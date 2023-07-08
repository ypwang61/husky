use super::*;
use husky_token::IdentToken;
use maybe_result::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_scope_resolution_ty(
        &mut self,
        expr_idx: ExprIdx,
        parent_expr_idx: ExprIdx,
        ident_token: IdentToken,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        self.infer_new_expr_ty_discarded(parent_expr_idx, ExpectEqsCategory::new_any_sort());
        let parent_term = self
            .infer_new_expr_term(parent_expr_idx)
            .ok_or(DerivedExprTypeError::UnableToInferAssociatedItemParentTerm)?;
        match parent_term.disambiguate_scope_resolution(
            self,
            ident_token.ident(),
            /*ad hoc */ &[],
        ) {
            JustOk(disambiguation) => match disambiguation {
                ScopeResolutionDisambiguation::AssociatedFn(ref signature) => {
                    let ty = signature.ty();
                    Ok((disambiguation.into(), Ok(ty)))
                }
            },
            JustErr(_) => todo!(),
            Nothing => todo!(),
        }
    }
}