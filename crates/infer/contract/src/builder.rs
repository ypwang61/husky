mod eager;
mod impl_locality;
mod lazy;

use std::sync::Arc;

use ast::{AstIter, AstKind};
use defn_head::FieldKind;
use entity_route::EntityRouteKind;
use fold::LocalStack;
use infer_decl::DeclQueryGroup;
use infer_entity_route::{EntityRouteSheet, InferEntityRoute};
use word::RootIdentifier;

use crate::*;

pub struct ContractSheetBuilder<'a> {
    db: &'a dyn InferContractSalsaQueryGroup,
    file: FilePtr,
    main_file: FilePtr,
    entity_route_sheet: Arc<EntityRouteSheet>,
    contract_sheet: ContractSheet,
    trait_uses: LocalStack<EntityRouteKind>,
}

impl<'a> InferEntityRoute for ContractSheetBuilder<'a> {
    fn decl_db(&self) -> &dyn DeclQueryGroup {
        self.db.upcast()
    }

    fn entity_route_sheet(&self) -> &EntityRouteSheet {
        &self.entity_route_sheet
    }
}

impl<'a> ContractSheetBuilder<'a> {
    pub(crate) fn new(
        db: &'a dyn InferContractSalsaQueryGroup,
        file: FilePtr,
        ty_sheet: Arc<EntityRouteSheet>,
    ) -> Self {
        Self {
            db,
            file,
            main_file: db.main_file(file).unwrap(),
            contract_sheet: ContractSheet::new(ty_sheet),
            trait_uses: LocalStack::new(),
            entity_route_sheet: db.entity_route_sheet(file).unwrap(),
        }
    }

    pub(crate) fn infer_all(&mut self, ast_iter: AstIter) {
        let arena = self.contract_sheet.ty_sheet.ast_text.arena.clone();
        self.enter_block();
        for item in ast_iter {
            match item.value {
                Ok(value) => match value.kind {
                    AstKind::TypeDefnHead { .. } | AstKind::EnumVariantDefnHead { .. } => {
                        item.children.map(|children| self.infer_all(children));
                    }
                    AstKind::MainDefn => {
                        let output_ty = self.db.global_output_ty(self.main_file).unwrap();
                        self.infer_morphism(output_ty, item.children.unwrap(), &arena)
                    }
                    AstKind::DatasetConfigDefnHead => self.infer_routine(
                        RootIdentifier::DatasetType.into(),
                        item.children.unwrap(),
                        &arena,
                    ),
                    AstKind::RoutineDefnHead(ref head) => {
                        self.infer_routine(head.output_ty.route, item.children.unwrap(), &arena)
                    }
                    AstKind::PatternDefnHead => todo!(),
                    AstKind::Use { ident, scope } => todo!(),
                    AstKind::FieldDefnHead(ref head) => match head.kind {
                        FieldKind::StructOriginal => (),
                        FieldKind::RecordOriginal => (),
                        FieldKind::StructDerived | FieldKind::RecordDerived => {
                            self.infer_morphism(head.ty, item.children.unwrap(), &arena)
                        }
                    },
                    AstKind::Stmt(_) => todo!(),
                    AstKind::TypeMethodDefnHead(ref head) => {
                        self.infer_routine(head.output_ty.route, item.children.unwrap(), &arena)
                    }
                    AstKind::FeatureDecl { ty, .. } => {
                        self.infer_morphism(ty.route, item.children.unwrap(), &arena)
                    }
                },
                _ => (),
            }
        }
        self.exit_block()
    }

    pub(crate) fn finish(self) -> ContractSheet {
        self.contract_sheet
    }
}
