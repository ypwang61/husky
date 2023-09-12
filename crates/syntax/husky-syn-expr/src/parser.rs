mod accept;
mod debug;
mod disambiguate_token;
mod env;
mod expr_stack;
mod incomplete_expr;

pub use self::env::*;

use self::disambiguate_token::*;
use self::expr_stack::*;
use self::incomplete_expr::*;
use crate::symbol::*;
use crate::*;
use husky_ast::{Ast, AstIdxRange, AstSheet};
use husky_entity_syn_tree::*;
use husky_vfs::{ModulePath, Toolchain};
use original_error::OriginalError;
use parsec::{HasStreamState, StreamParser};
use salsa::DebugWithDb;
use std::ops::ControlFlow;

#[macro_use]
macro_rules! report {
    ($self: expr) => {{
        p!(
            $self.stack,
            $self.parser.item_path.debug($self.db()) // $self.token_stream.text_range()
        );
    }};
}
use report;

/// parse token group into exprs
pub struct SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(crate) context: C,
    env_stack: ExprEnvironmentStack,
    token_stream: RegionalTokenStream<'a>,
    stack: ExprStack,
}

pub type SynDeclExprParser<'a> = SynExprParser<'a, SynExprContext<'a>>;
pub type SynDefnExprParser<'a, 'b> = SynExprParser<'a, &'b mut SynExprContext<'a>>;

impl<'a, C> HasTokenDb for SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    fn token_db(&self) -> &dyn TokenDb {
        self.context().db()
    }
}

impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(super) fn new(
        context: C,
        env: Option<ExprEnvironment>,
        token_stream: RegionalTokenStream<'a>,
    ) -> Self {
        Self {
            context,
            env_stack: ExprEnvironmentStack::new(env),
            token_stream,
            stack: Default::default(),
        }
    }

    pub(crate) fn context(&self) -> &SynExprContext<'a> {
        self.context.borrow()
    }

    pub(crate) fn context_mut(&mut self) -> &mut SynExprContext<'a> {
        self.context.borrow_mut()
    }

    pub(crate) fn db(&self) -> &'a dyn SynExprDb {
        self.context().db()
    }

    pub fn syn_expr_arena(&self) -> &SynExprArena {
        &self.context().syn_expr_arena()
    }

    pub fn parse_expr_root(
        &mut self,
        env: impl Into<Option<ExprEnvironment>>,
        expr_root_kind: ExprRootKind,
    ) -> Option<SynExprIdx> {
        let env = env.into();
        if let Some(env) = env {
            self.env_stack.set(env);
        }
        loop {
            let Some((token_idx, token)) = self.token_stream.next_indexed() else {
                break;
            };
            match self.disambiguate_token(token_idx, token) {
                ControlFlow::Continue(resolved_token) => self.accept_token(resolved_token),
                ControlFlow::Break(_) => {
                    self.rollback_raw(token_idx);
                    break;
                }
            }
        }
        self.reduce(Precedence::None);
        if env.is_some() {
            self.env_stack.unset();
        }
        let opt_expr_idx = self.finish_batch();
        opt_expr_idx.map(|expr_idx| self.context_mut().add_expr_root(expr_root_kind, expr_idx));
        opt_expr_idx
    }

    pub fn parse_expr_expected<E: OriginalError>(
        &mut self,
        env: Option<ExprEnvironment>,
        err: impl FnOnce(RegionalTokenStreamState) -> E,
    ) -> Result<SynExprIdx, E::Error> {
        let state = self.save_state();
        if let Some(env) = env {
            self.env_stack.set(env);
        }
        loop {
            let Some((token_idx, token)) = self.token_stream.next_indexed() else {
                break;
            };
            match self.disambiguate_token(token_idx, token) {
                ControlFlow::Continue(resolved_token) => self.accept_token(resolved_token),
                ControlFlow::Break(_) => {
                    self.rollback_raw(token_idx);
                    break;
                }
            }
        }
        self.reduce(Precedence::None);
        if env.is_some() {
            self.env_stack.unset();
        }
        match self.finish_batch() {
            Some(expr_idx) => Ok(expr_idx),
            None => Err(err(state).into()),
        }
    }

    pub fn parse_expr_expected2(
        &mut self,
        env: Option<ExprEnvironment>,
        expr_root_kind: ExprRootKind,
        err: impl FnOnce(RegionalTokenStreamState) -> OriginalSynExprError,
    ) -> SynExprIdx {
        let state = self.save_state();
        if let Some(env) = env {
            self.env_stack.set(env);
        }
        loop {
            let Some((token_idx, token)) = self.token_stream.next_indexed() else {
                break;
            };
            match self.disambiguate_token(token_idx, token) {
                ControlFlow::Continue(resolved_token) => self.accept_token(resolved_token),
                ControlFlow::Break(_) => {
                    self.rollback_raw(token_idx);
                    break;
                }
            }
        }
        self.reduce(Precedence::None);
        if env.is_some() {
            self.env_stack.unset();
        }
        let expr_idx = {
            match self.finish_batch() {
                Some(expr_idx) => expr_idx,
                None => self
                    .context_mut()
                    .alloc_expr(SynExpr::Err(err(state).into())),
            }
        };
        self.context_mut().add_expr_root(expr_root_kind, expr_idx);
        expr_idx
    }

    pub(crate) fn pattern_expr_region(&self) -> &SynPatternExprRegion {
        self.context().pattern_expr_region()
    }

    pub(crate) fn define_symbol(
        &mut self,
        variable: CurrentSynSymbol,
        ty_constraint: Option<ObeliskTypeConstraint>,
    ) -> CurrentSynSymbolIdx {
        self.context_mut().define_symbol(variable, ty_constraint)
    }

    pub(crate) fn define_symbols(
        &mut self,
        variables: impl IntoIterator<Item = CurrentSynSymbol>,
        ty_constraint: Option<ObeliskTypeConstraint>,
    ) -> CurrentSynSymbolIdxRange {
        self.context_mut().define_symbols(variables, ty_constraint)
    }

    pub fn parse_pattern_expr(
        &mut self,
        env: SynPatternExprInfo,
    ) -> SynExprResult<Option<SynPatternExprIdx>> {
        let symbol_modifier_token_group = self.try_parse_option()?;
        let ident_token = match symbol_modifier_token_group {
            None => match self.try_parse_option::<RegionalIdentToken>()? {
                Some(ident_token) => ident_token,
                None => return Ok(None),
            },
            Some(ephem_symbol_modifier_token_group) => {
                self.try_parse_expected(|token_stream_state| {
                    OriginalSynExprError::ExpectedIdentAfterModifier(
                        token_stream_state,
                        ephem_symbol_modifier_token_group,
                    )
                })?
            }
        };
        Ok(Some(self.context_mut().alloc_pattern_expr(
            SynPatternExpr::Ident {
                symbol_modifier_keyword_group: symbol_modifier_token_group,
                ident_token,
            },
            env,
        )))
        // if let Some(ref_token) = self.parse::<RefToken>()? {
        //     todo!()
        // } else if let Some(mut_token) = self.parse::<MutToken>()? {
        //     let ident_token: RegionalIdentToken =
        //         self.parse_expected(OriginalExprError::ExpectedIdentAfterMut)?;
        //     Ok(Some(self.alloc_pattern_expr(
        //         PatternExpr::Ident {
        //             ident_token,
        //             modifier: SymbolModifierKeywordGroup::Mut(mut_token),
        //         },
        //         env,
        //     )))
        // } else if let Some(ident_token) = self.parse::<RegionalIdentToken>()? {
        //     Ok(Some(self.alloc_pattern_expr(
        //         PatternExpr::Ident {
        //             ident_token,
        //             modifier: SymbolModifierKeywordGroup::Pure,
        //         },
        //         env,
        //     )))
        // } else {
        //     Ok(None)
        // }
    }

    fn allow_self_ty(&self) -> AllowSelfType {
        self.context()
            .syn_symbol_context()
            .symbol_region()
            .allow_self_ty()
    }

    fn allow_self_value(&self) -> AllowSelfValue {
        self.context()
            .syn_symbol_context()
            .symbol_region()
            .allow_self_value()
    }
}

impl<'a> SynExprParser<'a, SynExprContext<'a>> {
    pub fn finish(self) -> SynExprRegion {
        self.context.finish()
    }
}

impl<'a, C> std::ops::Deref for SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    type Target = RegionalTokenStream<'a>;
    fn deref(&self) -> &Self::Target {
        &self.token_stream
    }
}

impl<'a, C> std::ops::DerefMut for SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_stream
    }
}

impl<'a, C> std::borrow::Borrow<TokenStream<'a>> for SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_stream
    }
}

impl<'a, C> std::borrow::BorrowMut<TokenStream<'a>> for SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_stream
    }
}

impl<'a, C> parsec::StreamWrapper for SynExprParser<'a, C> where C: IsSynExprContext<'a> {}
