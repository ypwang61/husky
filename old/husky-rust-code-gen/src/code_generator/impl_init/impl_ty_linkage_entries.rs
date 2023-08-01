use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_ty_linkages(
        &mut self,
        opt_type_call: &Option<Arc<husky_item_semantics::TypeCallDefn>>,
        ty: EtherealTerm,
        members: &Arc<Vec<Arc<EntityDefn>>>,
    ) {
        if let Some(_) = opt_type_call {
            self.gen_type_call_linkage(ty);
        }
        // currently field and index are always generated
        self.gen_member_access_linkages(members, ty);
    }

    fn gen_type_call_linkage(&mut self, _item_route: EtherealTerm) {
        todo!()
        //         self.write("\n    (\n");
        //         self.write(&format!(
        //             r#"        __StaticLinkageKey::TypeCall {{
        //             ty: "{}"
        //         }},
        // "#,
        //             item_path
        //         ));
        //         let call_fugitive_syn_decl = self.db.item_call_fugitive_syn_decl(item_path).unwrap();
        //         self.gen_transfer_linkage(
        //             false, // ad hoc
        //             None,
        //             |this| {
        //                 this.gen_item_route(item_path, EntityRouteRole::Caller);
        //                 this.write("::__call__")
        //             },
        //             |this| {
        //                 this.gen_item_route(item_path, EntityRouteRole::StaticCallRoute);
        //                 this.write("::__call__")
        //             },
        //             &call_fugitive_syn_decl,
        //         );
        //         self.write("\n    ),");
    }

    fn gen_member_access_linkages(
        &mut self,
        members: &Arc<Vec<Arc<EntityDefn>>>,
        ty: EtherealTerm,
    ) {
        // todo: use decl rather than defn
        for member in members.iter() {
            match member.variant {
                EntityDefnVariant::TyField {
                    field_ty,
                    ref field_variant,
                    contract,
                    ..
                } => self.gen_struct_field_linkages(field_variant, member, contract, ty, field_ty),
                _ => {
                    let member_item_route = match member.base_route.variant {
                        EntityRouteVariant::TraitForTypeMember { trai, ident, .. } => {
                            if trai.variant == self.db.item_route_menu().std_ops_index_trai.variant
                            {
                                self.db
                                    .trai_for_ty_subroute(ty, trai, ident, Default::default())
                            } else {
                                continue;
                            }
                        }
                        _ => continue,
                    };
                    self.gen_linkage_entry(member_item_route, member);
                }
            }
        }
    }

    fn gen_struct_field_linkages(
        &mut self,
        field_variant: &FieldDefnVariant,
        member: &Arc<EntityDefn>,
        contract: MemberModifier,
        ty: EtherealTerm,
        field_ty: EtherealTerm,
    ) {
        match field_variant {
            FieldDefnVariant::StructOriginal
            | FieldDefnVariant::StructDefault { .. }
            | FieldDefnVariant::StructDerivedEager { .. } => {
                self.write("\n    (\n");
                let field_ident = member.ident.as_str();
                let canonical_field_ty = field_ty.canonicalize();
                let field_ty_canonical_kind = canonical_field_ty.kind();
                let field_ty_reg_memory_kind = self.db.reg_memory_kind(field_ty);
                self.write(&format!(
                    r#"        __StaticLinkageKey::StructField {{
            this_ty: "{ty}",
            field_ident: "{field_ident}",
        }},
        eager_field_linkage!(
            {contract},
            {field_ty_canonical_kind},
            {field_ty_reg_memory_kind},
            "#
                ));
                self.gen_item_route(ty, EntityRouteRole::Decl);
                // INTRINSIC_THIS_TY_VTABLE
                self.write(", __registration__::");
                self.write(&self.db.mangled_intrinsic_ty_vtable(ty));
                // INTRINSIC_FIELD_TY
                self.write(", ");
                self.gen_item_route(field_ty.intrinsic(), EntityRouteRole::Decl);
                // INTRINSIC_FIELD_TY_VTABLE
                self.write(", __registration__::");
                self.write(&self.db.mangled_intrinsic_ty_vtable(field_ty));
                self.write(format!(
                    r#",
            {field_ident}
        )
    ),"#,
                ));
            }
            FieldDefnVariant::StructDerivedLazy { ref defn_repr } => match **defn_repr {
                DefinitionRepr::LazyExpr { .. } => (),
                DefinitionRepr::LazyBlock { .. } => (),
                DefinitionRepr::FuncBlock { return_ty, .. } => {
                    let field_ident = member.ident.as_str();
                    self.write(&format!(
                        r#"
    (
        __StaticLinkageKey::StructField {{
            this_ty: "{ty}",
            field_ident: "{field_ident}",
        }},
        lazy_field_linkage!("#,
                    ));
                    self.gen_item_route(ty, EntityRouteRole::Decl);
                    // INTRINSIC_THIS_TY_VTABLE
                    self.write(", __registration__::");
                    self.write(&self.db.mangled_intrinsic_ty_vtable(ty));
                    // INTRINSIC_FIELD_TY
                    self.write(", ");
                    self.gen_item_route(return_ty.route.intrinsic(), EntityRouteRole::Decl);
                    // INTRINSIC_FIELD_TY_VTABLE
                    self.write(", __registration__::");
                    self.write(&self.db.mangled_intrinsic_ty_vtable(return_ty.route));
                    self.write(", ");
                    self.write(field_ident);
                    self.write(
                        r#")
    ),"#,
                    );
                }
                DefinitionRepr::ProcBlock { return_ty, .. } => {
                    let field_ident = member.ident.as_str();
                    self.write(&format!(
                        r#"
    (
        __StaticLinkageKey::StructField {{
            this_ty: "{ty}",
            field_ident: "{field_ident}",
        }},
        lazy_field_linkage!("#,
                    ));
                    self.gen_item_route(ty, EntityRouteRole::Decl);
                    // INTRINSIC_THIS_TY_VTABLE
                    self.write(", __registration__::");
                    self.write(&self.db.mangled_intrinsic_ty_vtable(ty));
                    // INTRINSIC_FIELD_TY
                    self.write(", ");
                    self.gen_item_route(return_ty.route.intrinsic(), EntityRouteRole::Decl);
                    // INTRINSIC_FIELD_TY_VTABLE
                    self.write(", __registration__::");
                    self.write(&self.db.mangled_intrinsic_ty_vtable(return_ty.route));
                    self.write(", ");
                    self.write(field_ident);
                    self.write(
                        r#")
    ),"#,
                    );
                }
            },
            FieldDefnVariant::RecordOriginal | FieldDefnVariant::RecordDerived { .. } => (),
        }
    }
}