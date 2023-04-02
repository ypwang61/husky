use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct RegularParameterDeclPattern {
    pattern: PatternExprIdx,
    variables: CurrentSymbolIdxRange,
    colon: ColonToken,
    ty: ExprIdx,
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for RegularParameterDeclPattern {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        if let Some(pattern) = ctx.parse_pattern_expr(PatternExprInfo::Parameter)? {
            let symbols = ctx.pattern_expr_region().pattern_symbol_map(pattern);
            let access_start = ctx.state();
            let variables = symbols
                .iter()
                .map(|(ident, pattern_symbol_idx)| {
                    CurrentSymbol::new(
                        access_start,
                        None,
                        CurrentSymbolVariant::ExplicitParameter {
                            ident: *ident,
                            pattern_symbol_idx: *pattern_symbol_idx,
                        },
                    )
                })
                .collect::<Vec<_>>();
            let colon = ctx.parse_expected(OriginalExprError::ExpectColon)?;
            let ty = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinBracket(Bracket::Par)),
                OriginalExprError::ExpectedParameterType,
            );
            let variables = ctx.define_symbols(
                variables,
                Some(PatternTypeConstraint::ExplicitParameter { pattern, ty }),
            );
            Ok(Some(RegularParameterDeclPattern {
                pattern,
                variables,
                colon,
                ty,
            }))
        } else {
            Ok(None)
        }
    }
}

impl RegularParameterDeclPattern {
    pub fn pattern_expr_idx(&self) -> PatternExprIdx {
        self.pattern
    }

    pub fn pattern(&self) -> PatternExprIdx {
        self.pattern
    }

    pub fn variables(&self) -> ArenaIdxRange<CurrentSymbol> {
        self.variables
    }

    pub fn colon(&self) -> ColonToken {
        self.colon
    }

    pub fn ty(&self) -> ExprIdx {
        self.ty
    }
}