use parsec::{parse_consecutive_list, HasStreamState};

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct RegularStructFieldDeclPattern {
    decorators: Vec<FieldDecorator>,
    visibility: Option<FieldVisibilityExpr>,
    ident_token: IdentToken,
    colon: ColonToken,
    ty_expr_idx: ExprIdx,
    initialization: Option<RegularStructFieldInitialization>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RegularStructFieldInitialization {
    Bind {
        colon_eq_token: ColonEqToken,
        value: ExprIdx,
    },
    Default {},
}

impl RegularStructFieldDeclPattern {
    pub fn ident(&self) -> Ident {
        self.ident_token.ident()
    }

    pub fn colon(&self) -> ColonToken {
        self.colon
    }

    pub fn ty_expr_idx(&self) -> ExprIdx {
        self.ty_expr_idx
    }
}

impl<'a, 'b> parsec::TryParseOptionalFromStream<ExprParseContext<'a, 'b>>
    for RegularStructFieldDeclPattern
{
    type Error = ExprError;

    fn try_parse_optional_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        let decorators = parse_consecutive_list(ctx)?;
        let visibility = ctx.try_parse_optional()?;
        let Some(ident_token) = ctx.try_parse_optional::<IdentToken>()? else {
                return Ok(None)
            };
        let colon: ColonToken = ctx.parse_expected(OriginalExprError::ExpectedColon)?;
        let ty_expr_idx = ctx.parse_expr_expected2(
            None,
            ExprRootKind::RegularStructFieldType { ident_token },
            OriginalExprError::ExpectedFieldType,
        );
        let initialization =
            if let Some(colon_eq_token) = ctx.try_parse_optional::<ColonEqToken>()? {
                Some(RegularStructFieldInitialization::Bind {
                    colon_eq_token,
                    value: ctx.parse_expr_expected2(
                        None,
                        ExprRootKind::FieldBindInitialValue { ty_expr_idx },
                        OriginalExprError::ExpectedValueForFieldBindInitialization,
                    ),
                })
            } else if let Some(_) = ctx.try_parse_optional::<EqToken>()? {
                todo!()
            } else {
                None
            };
        let access_start = ctx.save_state().next_token_idx();
        Ok(Some(RegularStructFieldDeclPattern {
            decorators,
            visibility,
            ident_token,
            colon,
            ty_expr_idx,
            initialization,
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct FieldDecorator {}

impl<'a, 'b> parsec::TryParseOptionalFromStream<ExprParseContext<'a, 'b>> for FieldDecorator {
    type Error = ExprError;

    fn try_parse_optional_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(at_token) = ctx.try_parse_optional::<AtToken>()? else {
            return Ok(None)
        };
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum FieldVisibilityExpr {
    Pub,
}

impl<'a, 'b> parsec::TryParseOptionalFromStream<ExprParseContext<'a, 'b>> for FieldVisibilityExpr {
    type Error = ExprError;

    fn try_parse_optional_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(pub_token) = ctx.try_parse_optional::<PubToken>()? else {
            return Ok(None)
        };
        let Some(lpar_token) = ctx.try_parse_optional::<LeftParenthesisToken>()? else {
            return Ok(Some(FieldVisibilityExpr::Pub))
        };
        todo!()
    }
}
