mod enum_variant;
mod impl_instantiate;
mod trait_impl;
mod vec;

use std::iter::Peekable;

use check_utils::should_eq;
use entity_kind::{EnumVariantKind, RoutineKind};
use print_utils::p;
pub use trait_impl::*;
pub use vec::*;

use crate::*;
use ast::AstIter;
use atom::symbol_proxy::{Symbol, SymbolKind};
use defn_head::*;
use entity_route::*;
pub use enum_variant::*;
use fold::LocalStack;
use map_collect::MapCollect;
use vec_dict::VecDict;
use vm::{OutputContract, TySignature};
use word::{IdentArcDict, IdentDict, RangedCustomIdentifier};

#[derive(Debug, PartialEq, Eq)]
pub struct TypeDecl {
    pub this_ty: EntityRoutePtr,
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub ty_members: IdentDict<TyMemberDecl>,
    pub variants: IdentDict<EnumVariantDecl>,
    pub kind: TyKind,
    pub trai_impls: Vec<Arc<TraiImplDecl>>,
    pub members: Vec<MemberDecl>,
    pub opt_type_call: Option<Arc<CallDecl>>,
}

impl TypeDecl {
    fn from_static(db: &dyn DeclQueryGroup, static_decl: &StaticTypeDecl) -> Arc<Self> {
        let generic_placeholders =
            db.parse_generic_placeholders_from_static(static_decl.generic_placeholders);
        let generic_arguments =
            db.generic_arguments_from_generic_placeholders(&generic_placeholders);
        let symbols = db.symbols_from_generic_placeholders(&generic_placeholders);
        let base_ty = db
            .parse_entity(static_decl.base_route, None, &symbols)
            .unwrap();
        let this_ty = db.intern_entity_route(EntityRoute {
            kind: base_ty.kind,
            generic_arguments,
        });
        let opt_type_call = static_decl
            .opt_type_call
            .map(|type_call| member_call_decl_from_static(db, symbols.clone(), type_call));
        Self::new(
            db,
            this_ty,
            generic_placeholders,
            static_decl
                .type_members
                .iter()
                .map(|member| TyMemberDecl::from_static(db, member, this_ty, &symbols))
                .collect(),
            static_decl.variants.map(|static_decl| {
                EnumVariantDecl::from_static(db, static_decl, this_ty, &symbols)
            }),
            static_decl.kind,
            static_decl
                .trait_impls
                .map(|trait_impl| TraiImplDecl::from_static(db, trait_impl, this_ty, &symbols)),
            opt_type_call,
        )
    }

    fn from_ast(
        db: &dyn DeclQueryGroup,
        arena: &RawExprArena,
        ty_route: EntityRoutePtr,
        kind: TyKind,
        generic_placeholders: IdentDict<GenericPlaceholder>,
        children: AstIter,
    ) -> InferResultArc<Self> {
        let generic_arguments =
            db.generic_arguments_from_generic_placeholders(&generic_placeholders);
        let this_ty = db.intern_entity_route(EntityRoute {
            kind: ty_route.kind,
            generic_arguments,
        });
        let mut children = children.peekable();
        let mut ty_members = IdentDict::default();
        let mut trai_impls = Vec::default();
        let variants = match kind {
            TyKind::Enum => Self::collect_variants(&mut children)?,
            _ => Default::default(),
        };
        Self::collect_original_fields(&mut children, &mut ty_members)?;
        Self::collect_other_members(children, &mut ty_members)?;
        let opt_type_call = match kind {
            TyKind::Enum => None,
            TyKind::Record | TyKind::Struct => {
                let mut inputs = vec![];
                for ty_member in ty_members.iter() {
                    match ty_member {
                        TyMemberDecl::Field(ref field_decl) => match field_decl.kind {
                            FieldKind::StructOriginal | FieldKind::RecordOriginal => {
                                inputs.push(InputDecl {
                                    contract: field_decl.contract.constructor_input(),
                                    ty: field_decl.ty,
                                    ident: field_decl.ident,
                                })
                            }
                            FieldKind::StructDerived | FieldKind::RecordDerived => break,
                        },
                        TyMemberDecl::Method(_) | TyMemberDecl::Call => break,
                    }
                }
                Some(Arc::new(CallDecl {
                    inputs,
                    output: OutputDecl {
                        ty: ty_route,
                        contract: OutputContract::Pure,
                    },
                    generic_placeholders: generic_placeholders.clone(),
                }))
            }
            TyKind::Primitive => todo!(),
            TyKind::Vec => panic!(),
            TyKind::Array => todo!(),
            TyKind::Other => todo!(),
        };
        Ok(TypeDecl::new(
            db,
            this_ty,
            generic_placeholders,
            ty_members,
            variants,
            kind,
            trai_impls,
            opt_type_call,
        ))
    }

    fn collect_variants(
        children: &mut Peekable<AstIter>,
    ) -> InferResult<IdentDict<EnumVariantDecl>> {
        let mut variants = VecDict::default();
        while let Some(child) = children.peek() {
            match child.value.as_ref()?.kind {
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class: ref raw_variant_kind,
                } => {
                    variants.insert_new(EnumVariantDecl {
                        ident,
                        variant: match raw_variant_kind {
                            EnumVariantKind::Constant => EnumVariantDeclVariant::Constant,
                        },
                    });
                    children.next();
                }
                _ => panic!(),
            }
        }
        Ok(variants)
    }

    fn collect_original_fields(
        children: &mut Peekable<AstIter>,
        members: &mut IdentDict<TyMemberDecl>,
    ) -> InferResult<()> {
        while let Some(child) = children.peek() {
            match child.value.as_ref()?.kind {
                AstKind::FieldDefnHead(ref field_defn_head) => {
                    match field_defn_head.kind {
                        FieldKind::StructOriginal | FieldKind::RecordOriginal => (),
                        FieldKind::StructDerived | FieldKind::RecordDerived => break,
                    }
                    children.next();
                    members.insert_new(TyMemberDecl::Field(FieldDecl::from_ast(field_defn_head)))
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn collect_other_members(
        mut children: Peekable<AstIter>,
        members: &mut IdentDict<TyMemberDecl>,
    ) -> InferResult<()> {
        while let Some(child) = children.next() {
            match child.value.as_ref()?.kind {
                AstKind::TypeDefnHead {
                    ident,
                    kind,
                    ref generic_placeholders,
                } => todo!(),
                AstKind::MainDefn => todo!(),
                AstKind::RoutineDefnHead(_) => todo!(),
                AstKind::PatternDefnHead => todo!(),
                AstKind::FeatureDecl { ident, ty } => todo!(),
                AstKind::TypeMethodDefnHead(ref method_defn_head) => {
                    match method_defn_head.routine_kind {
                        RoutineKind::Proc => todo!(),
                        RoutineKind::Func => members.insert_new(TyMemberDecl::Method(
                            MethodDecl::from_ast(method_defn_head, MethodKind::Type),
                        )),
                        RoutineKind::Test => todo!(),
                    }
                }
                AstKind::Use { ident, scope } => todo!(),
                AstKind::FieldDefnHead(ref field_defn_head) => match field_defn_head.kind {
                    FieldKind::StructOriginal => todo!("no original at this point"),
                    FieldKind::RecordOriginal => todo!("no original at this point"),
                    FieldKind::StructDerived | FieldKind::RecordDerived => members
                        .insert_new(TyMemberDecl::Field(FieldDecl::from_ast(field_defn_head))),
                },
                AstKind::DatasetConfigDefnHead => todo!(),
                AstKind::Stmt(_) => todo!(),
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class,
                } => todo!(),
            }
        }
        Ok(())
    }

    pub(crate) fn new(
        db: &dyn DeclQueryGroup,
        this_ty: EntityRoutePtr,
        generic_placeholders: IdentDict<GenericPlaceholder>,
        type_members: IdentDict<TyMemberDecl>,
        variants: IdentDict<EnumVariantDecl>,
        kind: TyKind,
        trait_impls: Vec<Arc<TraiImplDecl>>,
        opt_type_call: Option<Arc<CallDecl>>,
    ) -> Arc<Self> {
        let members = MemberDecl::collect_all(db, &type_members, &trait_impls);
        Arc::new(Self {
            this_ty,
            generic_placeholders,
            ty_members: type_members,
            variants,
            kind,
            trai_impls: trait_impls,
            members,
            opt_type_call,
        })
    }

    pub fn field_idx(&self, field_ident: CustomIdentifier) -> usize {
        self.ty_members.position(field_ident).unwrap()
    }

    pub fn fields(&self) -> impl Iterator<Item = &FieldDecl> {
        self.ty_members.iter().filter_map(|member| match member {
            TyMemberDecl::Field(field_decl) => Some(field_decl as &FieldDecl),
            _ => None,
        })
    }

    pub fn field_ty_result(
        &self,
        ranged_ident: RangedCustomIdentifier,
    ) -> InferResult<EntityRoutePtr> {
        match self.ty_members.get(ranged_ident.ident) {
            Some(type_member_decl) => match type_member_decl {
                TyMemberDecl::Field(field) => Ok(field.ty),
                TyMemberDecl::Method(_) => todo!(),
                TyMemberDecl::Call => todo!(),
            },
            None => {
                p!(self);
                p!(ranged_ident);
                todo!()
            }
        }
        // match self.kind {
        //     TyKind::Struct {
        //         fields: ref field_vars,
        //         ..
        //     } => ok_or!(
        //         field_vars.get(ranged_ident.ident),
        //         format!("no such member variable {}", &ranged_ident.ident),
        //         ranged_ident.range
        //     )
        //     .map(|signature| signature.ty),
        //     TyKind::Enum { ref variants } => todo!(),
        //     TyKind::Record {
        //         fields: ref field_vars,
        //         derived_fields: ref field_features,
        //     } => {
        //         if let Some(field_var) = field_vars.get(ranged_ident.ident) {
        //             Ok(field_var.ty)
        //         } else if let Some(field_feature) = field_features.get(ranged_ident.ident) {
        //             Ok(*field_feature)
        //         } else {
        //             todo!()
        //         }
        //     }
        //     TyKind::Vec { element_ty } => todo!(),
        // }
    }

    pub fn field_decl(&self, ranged_ident: RangedCustomIdentifier) -> InferResultArcRef<FieldDecl> {
        match self.ty_members.get(ranged_ident.ident) {
            Some(member_decl) => match member_decl {
                TyMemberDecl::Field(field) => Ok(field),
                TyMemberDecl::Method(_) => todo!(),
                TyMemberDecl::Call => todo!(),
            },
            None => todo!(),
        }
        // self.fields[field_ident]
        // match self.kind {
        //     TyKind::Struct {
        //         fields: ref field_vars,
        //         ..
        //     } => *field_vars.get(ranged_ident.ident).unwrap(),
        //     TyKind::Enum { ref variants } => todo!(),
        //     TyKind::Record {
        //         fields: ref field_vars,
        //         derived_fields: ref field_features,
        //     } => {
        //         if let Some(field_var) = field_vars.get(ranged_ident.ident) {
        //             *field_var
        //         } else if let Some(field_feature) = field_features.get(ranged_ident.ident) {
        //             FieldDecl {
        //                 contract: MembAccessContract::LazyOwn,
        //                 ty: *field_feature,
        //             }
        //         } else {
        //             todo!()
        //         }
        //     }
        //     TyKind::Vec { element_ty } => todo!(),
        // }
    }

    pub fn field_kind(&self, field_ident: CustomIdentifier) -> FieldKind {
        match self.ty_members.get(field_ident).unwrap() {
            TyMemberDecl::Field(field) => field.kind,
            _ => panic!(""),
        }
        // match self.kind {
        //     TyKind::Struct {
        //         fields: ref field_vars,
        //         methods: ref field_routines,
        //     } => {
        //         if field_vars.get(field_ident).is_some() {
        //             FieldAccessKind::StructMembVar
        //         } else {
        //             panic!("todo: memb feature of struct")
        //         }
        //     }
        //     TyKind::Enum { ref variants } => todo!(),
        //     TyKind::Record {
        //         fields: ref field_vars,
        //         derived_fields: ref field_features,
        //     } => {
        //         if field_vars.get(field_ident).is_some() {
        //             FieldAccessKind::RecordMemb
        //         } else if field_features.get(field_ident).is_some() {
        //             FieldAccessKind::RecordMemb
        //         } else {
        //             todo!()
        //         }
        //     }
        //     TyKind::Vec { element_ty } => todo!(),
        // }
    }

    pub fn signature(&self) -> TySignature {
        todo!()
        // match self.kind {
        //     TyKind::Struct {
        //         fields: ref field_vars,
        //         ..
        //     } => {
        //         let mut vm_field_vars = IdentDict::<MembAccessContract>::default();
        //         field_vars.iter().for_each(|(ident, field_var_sig)| {
        //             vm_field_vars.insert_new(*ident, field_var_sig.contract)
        //         });
        //         TySignature::Struct {
        //             field_vars: vm_field_vars,
        //         }
        //     }
        //     TyKind::Enum { ref variants } => todo!(),
        //     TyKind::Record {
        //         fields: ref field_vars,
        //         derived_fields: ref field_features,
        //     } => todo!(),
        //     TyKind::Vec { element_ty } => TySignature::Vec,
        // }
    }

    pub fn method(
        &self,
        ranged_ident: RangedCustomIdentifier,
        trait_uses: &[EntityRouteKind],
    ) -> InferResult<&Arc<MethodDecl>> {
        // the rule is:
        // first look in the type members,
        // then look in the trait members,
        // if multiple are found in the trait members,
        // report an infer error.
        if let Some(member) = self.ty_members.get(ranged_ident.ident) {
            match member {
                TyMemberDecl::Field(_) => todo!(),
                TyMemberDecl::Method(method) => return Ok(method),
                TyMemberDecl::Call => todo!(),
            }
        }
        let matched_methods: Vec<&Arc<MethodDecl>> = self
            .members
            .iter()
            .enumerate()
            .filter_map(|(member_idx, member)| {
                if member.ident() == ranged_ident.ident {
                    match member {
                        MemberDecl::AssociatedType => todo!(),
                        MemberDecl::AssociatedCall => todo!(),
                        MemberDecl::TypeField(_) => todo!(),
                        MemberDecl::TypeMethod(_) => todo!(),
                        MemberDecl::TraitMethod {
                            trait_route,
                            method,
                        } => {
                            if is_trait_availabe(*trait_route, trait_uses) {
                                Some(method)
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            })
            .collect();
        if matched_methods.len() == 1 {
            return Ok(matched_methods[0]);
        } else {
            p!(ranged_ident);
            p!(self.ty_members);
            p!(matched_methods.len());
            todo!()
        }
        // ok_or!(
        //     self.type_members.get(ranged_ident.ident),
        //     format!(
        //         "no method named `{}` found in type `{:?}`",
        //         &ranged_ident.ident, self.this_ty
        //     ),
        //     ranged_ident.range
        // )
    }

    pub fn member_idx(&self, member_route: EntityRoutePtr) -> MemberIdx {
        match member_route.kind {
            EntityRouteKind::Child { parent, ident } => {
                should_eq!(self.this_ty, parent);
                self.ty_members.position(ident).unwrap().into()
            }
            EntityRouteKind::TraitMember { ty, trai, ident } => {
                should_eq!(self.this_ty, ty);
                todo!()
            }
            _ => panic!(),
        }
    }
}

pub(crate) fn type_decl(
    db: &dyn DeclQueryGroup,
    ty_route: EntityRoutePtr,
) -> InferResultArc<TypeDecl> {
    let source = db.entity_source(ty_route)?;
    match source {
        EntitySource::StaticModuleItem(data) => Ok(match data.decl {
            StaticEntityDecl::Func(_) => todo!(),
            StaticEntityDecl::Module => todo!(),
            StaticEntityDecl::Type(type_decl) => {
                let base_decl = TypeDecl::from_static(db, type_decl);
                assert_eq!(
                    ty_route.generic_arguments.len(),
                    base_decl.generic_placeholders.len()
                );
                if ty_route.generic_arguments.len() > 0 {
                    base_decl.instantiate(db, &ty_route.generic_arguments)
                } else {
                    base_decl
                }
            }
            StaticEntityDecl::Trait { .. } => todo!(),
        }),
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let item = ast_text
                .folded_results
                .fold_iter(token_group_index)
                .next()
                .unwrap();
            let ast = item.value.as_ref()?;
            match ast.kind {
                AstKind::TypeDefnHead {
                    kind,
                    ref generic_placeholders,
                    ..
                } => {
                    if ty_route.generic_arguments.len() > 0 {
                        todo!()
                    } else {
                        TypeDecl::from_ast(
                            db,
                            &ast_text.arena,
                            ty_route,
                            kind,
                            generic_placeholders.clone(),
                            item.children.unwrap(),
                        )
                    }
                }
                _ => panic!(),
            }
        }
        EntitySource::Module { file } => todo!(),
        EntitySource::Input { .. } => todo!(),
        EntitySource::StaticTypeMember => todo!(),
    }
}

fn is_trait_availabe(trait_route: EntityRoutePtr, trait_uses: &[EntityRouteKind]) -> bool {
    match trait_route.kind {
        EntityRouteKind::Root { ident } => true,
        EntityRouteKind::Package { main, ident } => todo!(),
        EntityRouteKind::Child { parent, ident } => todo!(),
        EntityRouteKind::Input { main } => todo!(),
        EntityRouteKind::Generic { ident, entity_kind } => todo!(),
        EntityRouteKind::ThisType => todo!(),
        EntityRouteKind::TraitMember {
            ty: parent,
            trai,
            ident,
        } => todo!(),
    }
}

pub(crate) fn member_call_decl_from_static(
    db: &dyn DeclQueryGroup,
    mut symbols: Vec<Symbol>,
    static_decl: &StaticCallDecl,
) -> Arc<CallDecl> {
    let generic_placeholders =
        db.parse_generic_placeholders_from_static(static_decl.generic_placeholders);
    symbols.extend(db.symbols_from_generic_placeholders(&generic_placeholders));
    let inputs = static_decl.inputs.map(|input| InputDecl {
        ty: db.parse_entity(input.ty, None, &symbols).unwrap(),
        contract: input.contract,
        ident: db.custom_ident(input.name),
    });
    let output_ty = db
        .parse_entity(static_decl.output_ty, None, &symbols)
        .unwrap();
    Arc::new(CallDecl {
        generic_placeholders,
        inputs,
        output: OutputDecl {
            contract: static_decl.output_contract,
            ty: output_ty,
        },
    })
}
