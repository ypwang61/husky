use super::*;
use husky_symbol::Symbol;
use husky_term::Term;
use husky_token::SpecialToken;
use std::ops::ControlFlow;

pub type TokenResolveResult<T> = ControlFlow<ExprParsingStopReason, T>;

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(crate) fn resolve_token(
        &self,
        token_idx: TokenIdx,
        token: &Token,
    ) -> TokenResolveResult<ResolvedToken> {
        TokenResolveResult::Continue(ResolvedToken {
            token_idx,
            kind: self.resolve_token_kind(token)?,
        })
    }

    fn resolve_token_kind(&self, token: &Token) -> TokenResolveResult<ResolvedTokenKind> {
        TokenResolveResult::Continue(match token.kind {
            TokenKind::Attr(_) => todo!(),
            TokenKind::Keyword(_keyword) => todo!(),
            TokenKind::Identifier(ident) => self.resolve_ident(ident),
            TokenKind::Special(special) => match special {
                SpecialToken::BinaryOpr(opr) => ResolvedTokenKind::BinaryOpr(opr),
                SpecialToken::Bra(bra) => ResolvedTokenKind::Bra(bra),
                SpecialToken::Ket(ket) => ResolvedTokenKind::Ket(ket),
                SpecialToken::LAngle => todo!(),
                SpecialToken::RAngle => todo!(),
                SpecialToken::DeriveAssign => todo!(),
                SpecialToken::Minus => ResolvedTokenKind::Prefix(PrefixOpr::Minus),
                SpecialToken::Exclamation => ResolvedTokenKind::Prefix(PrefixOpr::Not),
                SpecialToken::Incr => ResolvedTokenKind::Suffix(RawSuffixOpr::Incr),
                SpecialToken::Decr => ResolvedTokenKind::Suffix(RawSuffixOpr::Decr),
                SpecialToken::DoubleVertical => todo!(),
                SpecialToken::BitNot => todo!(),
                SpecialToken::Dot => ResolvedTokenKind::Dot,
                SpecialToken::BinaryOpr(BinaryOpr::Curry) => todo!(),
                SpecialToken::BinaryOpr(BinaryOpr::ScopeResolution) => todo!(),
                SpecialToken::Colon => todo!(),
                SpecialToken::Comma => todo!(),
                SpecialToken::Ambersand => todo!(),
                SpecialToken::Vertical => todo!(),
                SpecialToken::DoubleExclamation => todo!(),
                SpecialToken::Semicolon => {
                    return TokenResolveResult::Break(ExprParsingStopReason::Semicolon)
                }
                SpecialToken::XmlKet => todo!(),
                SpecialToken::At => todo!(),
                SpecialToken::QuestionMark => todo!(),
                SpecialToken::PoundSign => todo!(),
            },
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::Literal(ref literal) => ResolvedTokenKind::Atom(literal.clone().into()),
            TokenKind::Comment => todo!(),
            TokenKind::Err(_) => todo!(),
        })
    }

    fn resolve_ident(&self, ident: Identifier) -> ResolvedTokenKind {
        todo!()
        // if let Some(opr) = self.stack.top_opr() {
        //     match opr.variant {
        //         OnStackOprVariant::Binary(BinaryOpr::ScopeResolution) => {
        //             if let Some(previous_expr) = self.stack.top_expr() {
        //                 match previous_expr.base_scope_result() {
        //                     BaseScopeResult::None => todo!(),
        //                     BaseScopeResult::Some(_) => todo!(),
        //                     BaseScopeResult::Uncertain => {
        //                         todo!()
        //                         // return ResolvedTokenKind::Atom(AtomExpr::Uncertain(ident))
        //                     }
        //                 }
        //             } else {
        //                 todo!()
        //             }
        //         }
        //         _ => (),
        //     }
        // }
        // match self.symbols.resolve_ident(ident) {
        //     Some(symbol) => symbol.into(),
        //     None => ResolvedTokenKind::Atom(AtomExpr::Unrecognized(ident)),
        // }
    }

    fn resolve_previous_entity(&self) -> Option<Term> {
        self.top_expr().map(|expr| self.resolve_entity(expr))
    }

    fn resolve_entity(&self, expr: &Expr) -> Term {
        todo!()
        // match expr.variant {
        //     Expr::Atom(ref atom) => match atom {
        //         AtomExpr::Literal(_) => todo!(),
        //         AtomExpr::Symbol(_) => todo!(),
        //         AtomExpr::Uncertain(_) => todo!(),
        //         AtomExpr::Unrecognized(_) => todo!(),
        //     },
        //     Expr::Opn {
        //         ref opn_variant,
        //         ref opds,
        //     } => match opn_variant {
        //         RawOpnVariant::Binary(_) => todo!(),
        //         RawOpnVariant::Prefix(_) => todo!(),
        //         RawOpnVariant::Suffix(_) => todo!(),
        //         RawOpnVariant::List(_) => todo!(),
        //         RawOpnVariant::Field(_) => todo!(),
        //         RawOpnVariant::CurlBracketed => self.resolve_entity(&self.arena[opds.start()]),
        //         RawOpnVariant::Abstraction => todo!(),
        //     },
        // }
    }
}

#[derive(Clone)]
pub(crate) struct ResolvedToken {
    kind: ResolvedTokenKind,
    token_idx: TokenIdx,
}

impl ResolvedToken {
    pub(super) fn kind(&self) -> &ResolvedTokenKind {
        &self.kind
    }

    pub(super) fn to_expr(self, arena: &ExprArena) -> Expr {
        todo!()
        // let variant = match self.kind {
        //     ResolvedTokenKind::Atom(variant) => variant.into(),
        //     ResolvedTokenKind::BinaryOpr(_) => todo!(),
        //     ResolvedTokenKind::Prefix(_) => todo!(),
        //     ResolvedTokenKind::Suffix(_) => todo!(),
        // };
        // Expr::new(variant, self.range, arena)
    }

    pub(crate) fn token_idx(&self) -> TokenIdx {
        self.token_idx
    }
}

#[derive(Clone)]
pub(crate) enum ResolvedTokenKind {
    Atom(AtomExpr),
    BinaryOpr(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(RawSuffixOpr),
    Bra(Bracket),
    Ket(Bracket),
    Dot,
}

impl From<Symbol> for ResolvedTokenKind {
    fn from(symbol: Symbol) -> Self {
        ResolvedTokenKind::Atom(symbol.into())
    }
}