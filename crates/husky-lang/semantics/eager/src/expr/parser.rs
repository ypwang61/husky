use ast::{RawExprArena, RawExprIdx, RawExprKind, RawExprRange};
use file::FilePtr;
use scope::{ScopeKind, ScopePtr};
use syntax_types::{ListOpr, Opr, SuffixOpr};
use vm::{BinaryOpr, EagerContract, PrimitiveValue};
use word::BuiltinIdentifier;

use crate::*;
use semantics_error::{err, try_infer};

use super::EagerOpnKind;

pub trait EagerExprParser<'a> {
    fn arena(&self) -> &'a RawExprArena;
    fn db(&self) -> &'a dyn InferQueryGroup;
    fn file(&self) -> FilePtr;

    fn parse_eager_expr(&mut self, raw_expr_idx: RawExprIdx) -> SemanticResult<Arc<EagerExpr>> {
        let raw_expr = &self.arena()[raw_expr_idx];
        let kind = match raw_expr.kind {
            RawExprKind::Variable { varname, .. } => EagerExprKind::Variable(varname),
            RawExprKind::Unrecognized(ident) => {
                err!(format!(
                    "unrecognized identifier {} at {}:{:?}",
                    ident,
                    self.file().to_str().unwrap(),
                    raw_expr.range()
                ))
            }
            RawExprKind::Scope { scope, kind } => match kind {
                ScopeKind::Module => todo!(),
                ScopeKind::Literal => match scope {
                    ScopePtr::Builtin(BuiltinIdentifier::True) => {
                        EagerExprKind::Literal(PrimitiveValue::Bool(true))
                    }
                    ScopePtr::Builtin(BuiltinIdentifier::False) => {
                        EagerExprKind::Literal(PrimitiveValue::Bool(false))
                    }
                    ScopePtr::Custom(_) => todo!(),
                    _ => todo!(),
                },
                ScopeKind::Type => todo!(),
                ScopeKind::Trait => todo!(),
                ScopeKind::Routine => todo!(),
                ScopeKind::Feature => {
                    panic!("what")
                }
                ScopeKind::Pattern => todo!(),
            },
            RawExprKind::PrimitiveLiteral(value) => EagerExprKind::Literal(value),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn { opr, ref opds } => self.parse_opn(opr, opds)?,
            RawExprKind::Lambda(_, _) => todo!(),
            RawExprKind::This { .. } => EagerExprKind::This,
        };
        Ok(Arc::new(EagerExpr {
            range: raw_expr.range().clone(),
            ty: try_infer!(self.db().expr_ty_result(self.file(), raw_expr_idx)),
            kind,
            file: self.file(),
            instruction_id: Default::default(),
            contract: try_infer!(self
                .db()
                .eager_expr_contract_result(self.file(), raw_expr_idx)),
        }))
    }

    fn parse_opn(&mut self, opr: Opr, opds: &RawExprRange) -> SemanticResult<EagerExprKind> {
        match opr {
            Opr::Binary(opr) => self.parse_binary_opr(opr, opds),
            Opr::Prefix(_) => todo!(),
            Opr::Suffix(opr) => self.parse_suffix_opr(opr, opds),
            Opr::List(opr) => match opr {
                ListOpr::TupleInit => todo!(),
                ListOpr::NewVec => todo!(),
                ListOpr::NewDict => todo!(),
                ListOpr::Call => self.parse_call(opds.clone()),
                ListOpr::Index => todo!(),
                ListOpr::ModuloIndex => todo!(),
                ListOpr::StructInit => todo!(),
            },
        }
    }

    fn parse_binary_opr(
        &mut self,
        opr: BinaryOpr,
        raw_opd_idx_range: &RawExprRange,
    ) -> SemanticResult<EagerExprKind> {
        let raw_opds = &self.arena()[raw_opd_idx_range];
        let lopd = self.parse_eager_expr(raw_opd_idx_range.start)?;
        let ropd = self.parse_eager_expr(raw_opd_idx_range.start + 1)?;
        Ok(EagerExprKind::Opn {
            opn_kind: EagerOpnKind::Binary { opr, this: lopd.ty },
            opds: vec![lopd, ropd],
            compiled: None,
        })
    }

    fn parse_suffix_opr(
        &mut self,
        opr: SuffixOpr,
        raw_opds: &RawExprRange,
    ) -> SemanticResult<EagerExprKind> {
        let opd_idx = raw_opds.start;
        let opd = self.parse_eager_expr(opd_idx)?;
        Ok(EagerExprKind::Opn {
            opn_kind: EagerOpnKind::Suffix { opr, this: opd.ty },
            opds: vec![opd],
            compiled: None,
        })
    }

    fn parse_call(&mut self, opd_idx_range: RawExprRange) -> SemanticResult<EagerExprKind> {
        let call = &self.arena()[opd_idx_range.start];
        let input_opd_idx_range = (opd_idx_range.start + 1)..opd_idx_range.end;
        match call.kind {
            RawExprKind::Scope {
                scope,
                kind: ScopeKind::Routine,
                ..
            } => {
                let signature = try_infer!(self.db().call_signature(scope));
                let arguments: Vec<_> = input_opd_idx_range
                    .clone()
                    .enumerate()
                    .map(|(i, raw)| self.parse_eager_expr(raw))
                    .collect::<SemanticResult<_>>()?;
                let output = signature.output;
                Ok(EagerExprKind::Opn {
                    opn_kind: EagerOpnKind::RoutineCall(RangedScope {
                        scope,
                        range: call.range(),
                    }),
                    compiled: signature.compiled,
                    opds: arguments,
                })
            }
            RawExprKind::Scope {
                scope,
                kind: ScopeKind::Type,
                ..
            } => {
                let signature = try_infer!(self.db().call_signature(scope));
                let arguments: Vec<_> = input_opd_idx_range
                    .enumerate()
                    .map(|(i, raw)| self.parse_eager_expr(raw))
                    .collect::<SemanticResult<_>>()?;
                Ok(EagerExprKind::Opn {
                    opn_kind: EagerOpnKind::TypeCall {
                        ranged_ty: RangedScope {
                            scope,
                            range: call.range(),
                        },
                        ty_signature: try_infer!(self.db().ty_signature(scope)),
                    },
                    compiled: signature.compiled,
                    opds: arguments,
                })
            }
            RawExprKind::Scope { .. } => todo!(),
            RawExprKind::Variable { .. } => todo!(),
            RawExprKind::Unrecognized(_) => todo!(),
            RawExprKind::PrimitiveLiteral(_) => todo!(),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn { opr, ref opds } => todo!(),
            RawExprKind::Lambda(_, _) => todo!(),
            RawExprKind::This { .. } => todo!(),
        }
    }
}
