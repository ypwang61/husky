mod explicit_application;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_expr_term(&mut self, expr_idx: ExprIdx) -> Option<LocalTerm> {
        // expect to infer type before infer term
        #[cfg(test)]
        if self.expr_ty_infos.get(expr_idx).is_none() {
            print_debug_expr!(self, expr_idx);
            panic!()
        }
        let term_result = self.calc_expr_term(expr_idx);
        let term = term_result.as_ref().ok().copied();
        self.save_new_expr_term(expr_idx, term_result);
        term
    }

    fn save_new_expr_term(&mut self, expr_idx: ExprIdx, term_result: ExprTermResult<LocalTerm>) {
        self.expr_terms.insert_new(expr_idx, term_result)
    }

    fn calc_expr_term(&mut self, expr_idx: ExprIdx) -> ExprTermResult<LocalTerm> {
        match self.expr_region_data[expr_idx] {
            Expr::Literal(_) => todo!(),
            Expr::EntityPath {
                entity_path_expr,
                entity_path,
            } => match entity_path {
                Some(entity_path) => Ok(self.db.reduced_term(entity_path.into()).into()),
                None => todo!(),
            },
            Expr::InheritedSymbol {
                ident,
                token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => todo!(),
            Expr::CurrentSymbol {
                ident,
                token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            Expr::FrameVarDecl {
                token_idx,
                ident,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            Expr::SelfType(_) => todo!(),
            Expr::SelfValue(_) => todo!(),
            Expr::Binary {
                lopd,
                opr,
                opr_token_idx,
                ropd,
            } => {
                p!(self.path(), opr_token_idx);
                p!(opr);
                todo!()
            }
            Expr::Be { .. } => todo!(),
            Expr::Prefix {
                opr,
                opr_token_idx,
                opd,
            } => todo!(),
            Expr::Suffix {
                opd,
                opr: punctuation,
                opr_token_idx: punctuation_token_idx,
            } => todo!(),
            Expr::ExplicitApplicationOrRitchieCall { .. } => todo!(),
            Expr::Field {
                owner,
                dot_token_idx,
                ident_token,
            } => todo!(),
            Expr::MethodCall { .. } => todo!(),
            Expr::TemplateInstantiation { .. } => todo!(),
            Expr::ExplicitApplicationOrComposition { function, argument } => {
                self.calc_explicit_application_expr_term(function, argument)
            }
            Expr::Bracketed {
                lpar_token_idx,
                item,
                rpar_token_idx,
            } => todo!(),
            Expr::NewTuple { .. } => todo!(),
            Expr::BoxList { .. } => todo!(),
            Expr::BoxColonList { .. } => todo!(),
            Expr::Block { stmts } => todo!(),
            Expr::IndexOrCompositionWithList {
                owner,
                lbox_token_idx,
                items: indices,
                rbox_token_idx,
            } => todo!(),
            Expr::Err(_) => Err(DerivedExprTermError::ExprError.into()),
        }
    }
}
