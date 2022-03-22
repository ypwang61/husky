use crate::*;
use common::p;
use scope::ScopePtr;
use vm::{Instruction, InstructionSheet};

#[salsa::query_group(InstructionQueryGroupStorage)]
pub trait InstructionQueryGroup: EntityQueryGroup {
    fn instruction_sheet(&self, scope: ScopePtr) -> SemanticResultArc<InstructionSheet>;
}

fn instruction_sheet(
    this: &dyn InstructionQueryGroup,
    scope: ScopePtr,
) -> SemanticResultArc<InstructionSheet> {
    let entity = this.entity(scope)?;
    Ok(match entity.kind() {
        EntityKind::Module { .. } => todo!(),
        EntityKind::Feature(_) => todo!(),
        EntityKind::Pattern { .. } => todo!(),
        EntityKind::Func {
            input_placeholders,
            stmts,
            ..
        } => InstructionSheetBuilder::new_decl(
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident)
                .collect(),
            stmts,
        ),
        EntityKind::Proc {
            input_placeholders,
            stmts,
            ..
        } => InstructionSheetBuilder::new_impr(
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident)
                .collect(),
            stmts,
        ),
        EntityKind::Ty(_) => todo!(),
    })
}
