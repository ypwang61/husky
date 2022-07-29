use crate::*;
use entity_kind::TyKind;
use husky_ast::FieldAstKind;
use husky_linkage_table::ResolveLinkage;
use husky_primitive_literal_semantics::convert_primitive_literal_to_value;
use husky_vm_primitive_opr_linkage::{
    resolve_primitive_assign_binary_opr_linkage, resolve_primitive_pure_binary_opr_linkage,
};
use infer_decl::TyDecl;
use map_collect::MapCollect;
use vm::{
    __root::{__EQ_LINKAGE, __NEQ_LINKAGE, __VALUE_CALL_LINKAGE},
    *,
};

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_eager_expr(
        &mut self,
        expr: &Arc<EagerExpr>,
        output_stack_idx: VMStackIdx,
    ) {
        match expr.variant {
            EagerExprVariant::Variable { varname, binding } => {
                let stack_idx = self.sheet.variable_stack.stack_idx(varname);
                self.push_instruction(Instruction::new(
                    InstructionVariant::PushVariable {
                        varname: varname.into(),
                        stack_idx,
                        binding,
                        range: expr.range,
                        ty: expr.ty(),
                        explicit: true,
                    },
                    expr.clone(),
                ))
            }
            EagerExprVariant::PrimitiveLiteral(value) => self.push_instruction(Instruction::new(
                InstructionVariant::PushPrimitiveValue {
                    value: convert_primitive_literal_to_value(value, expr.ty()),
                    explicit: true,
                    ty: expr.ty(),
                },
                expr.clone(),
            )),
            EagerExprVariant::Bracketed(ref expr) => {
                self.compile_eager_expr(expr, output_stack_idx)
            }
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.compile_opn(opn_variant, opds, expr, output_stack_idx),
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::ThisValue { binding } => self.push_instruction(Instruction::new(
                InstructionVariant::PushVariable {
                    varname: ContextualIdentifier::ThisValue.into(),
                    stack_idx: VMStackIdx::this(),
                    binding,
                    range: expr.range,
                    ty: expr.ty(),
                    explicit: true,
                },
                expr.clone(),
            )),
            EagerExprVariant::ThisField {
                field_ident,
                field_idx,
                this_ty,
                this_binding,
                field_binding,
            } => match self.context.value() {
                InstructionGenContext::Normal => {
                    self.push_instruction(Instruction::new(
                        InstructionVariant::PushVariable {
                            varname: ContextualIdentifier::ThisValue.into(),
                            stack_idx: VMStackIdx::this(),
                            binding: this_binding,
                            range: expr.range,
                            ty: this_ty,
                            explicit: false,
                        },
                        expr.clone(),
                    ));
                    self.push_instruction(Instruction::new(
                        if let Some(linkage) = self.db.compile_time().struct_field_access_linkage(
                            this_ty,
                            field_ident.ident,
                            field_binding,
                        ) {
                            InstructionVariant::CallRoutine {
                                output_ty: expr.ty(),
                                nargs: 1,
                                linkage_fp: linkage,
                            }
                        } else {
                            let this_ty_decl = self.db.compile_time().ty_decl(this_ty).unwrap();
                            InstructionVariant::FieldAccessInterpreted {
                                field_idx: this_ty_decl
                                    .field_idx(field_ident.ident)
                                    .try_into()
                                    .unwrap(),
                                field_binding,
                                field_ty: expr.ty(),
                            }
                        },
                        expr.clone(),
                    ))
                }
                InstructionGenContext::NewVirtualStruct { output_stack_idx } => self
                    .push_instruction(Instruction::new(
                        InstructionVariant::PushVariable {
                            varname: field_ident.ident.into(),
                            stack_idx: output_stack_idx + field_idx,
                            binding: field_binding,
                            range: expr.range,
                            ty: expr.ty(),
                            explicit: true,
                        },
                        expr.clone(),
                    )),
            },
            EagerExprVariant::EnumKindLiteral(route) => self.push_instruction(Instruction::new(
                InstructionVariant::PushEnumKindLiteral(EnumKindValue {
                    kind_idx: self.db.enum_literal_as_u8(route),
                    route,
                }),
                expr.clone(),
            )),
            EagerExprVariant::EntityFeature { route } => self.push_instruction(Instruction::new(
                InstructionVariant::EntityFeature {
                    feature_uid: self.db.compile_time().entity_uid(route),
                    ty: expr.ty(),
                },
                expr.clone(),
            )),
            EagerExprVariant::EntityFp { route } => self.push_instruction(Instruction::new(
                InstructionVariant::PushEntityFp {
                    opt_linkage: self.db.compile_time().routine_linkage(route),
                    opt_instruction_sheet: self.db.entity_instruction_sheet(route),
                    ty: expr.ty(),
                },
                expr.clone(),
            )),
        }
    }

    fn compile_opn(
        &mut self,
        opn_variant: &EagerOpnVariant,
        opds: &[Arc<EagerExpr>],
        expr: &Arc<EagerExpr>,
        output_stack_idx: VMStackIdx,
    ) {
        for (i, opd) in opds.iter().enumerate() {
            self.compile_eager_expr(opd, output_stack_idx + i);
        }
        match opn_variant {
            EagerOpnVariant::Binary { opr, this_ty } => {
                self.compile_binary_opn(*opr, this_ty, opds, expr)
            }
            EagerOpnVariant::Prefix { opr, this_ty } => {
                todo!()
                // let instruction = Instruction::new(
                //     InstructionVariant::OprOpn {
                //         opn: OprOpn::Prefix(*opr),
                //         this_ty: *this_ty,
                //     },
                //     expr.clone(),
                // );
                // self.push_instruction(instruction)
            }
            EagerOpnVariant::Suffix { opr, this_ty } => {
                let ins_kind = match opr {
                    SuffixOpr::Incr => {
                        todo!()
                        //     InstructionVariant::OprOpn {
                        //     opn: OprOpn::Incr,
                        //     this_ty: *this_ty,
                        // }
                    }
                    SuffixOpr::Decr => {
                        todo!()
                        //     InstructionVariant::OprOpn {
                        //     opn: OprOpn::Decr,
                        //     this_ty: *this_ty,
                        // }
                    }
                    SuffixOpr::AsTy(ty) => {
                        todo!()
                        //     InstructionVariant::OprOpn {
                        //     opn: match ty.route {
                        //         EntityRoutePtr::Root(ty_ident) => match ty_ident {
                        //             RootIdentifier::Void => todo!(),
                        //             RootIdentifier::Bool => todo!(),
                        //             RootIdentifier::B32 => OprOpn::Cast(CastOpn::AsB32),
                        //             RootIdentifier::B64 => OprOpn::Cast(CastOpn::AsB64),
                        //             RootIdentifier::I32 => OprOpn::Cast(CastOpn::AsI32),
                        //             RootIdentifier::I64 => OprOpn::Cast(CastOpn::AsI64),
                        //             RootIdentifier::F32 => OprOpn::Cast(CastOpn::AsF32),
                        //             RootIdentifier::F64 => OprOpn::Cast(CastOpn::AsF64),
                        //             _ => todo!(),
                        //         },
                        //         EntityRoutePtr::Custom(_) => todo!(),
                        //         EntityRoutePtr::ThisType => todo!(),
                        //     },
                        //     this_ty: *this_ty,
                        // }
                    }
                };
                let instruction = Instruction::new(ins_kind, expr.clone());
                self.push_instruction(instruction)
            }
            EagerOpnVariant::RoutineCall(routine) => {
                if let Some(__Linkage) = self.db.compile_time().routine_linkage(routine.route) {
                    match __Linkage {
                        __Linkage::Member { .. } => todo!(),
                        __Linkage::Transfer(linkage) => self.push_instruction(Instruction::new(
                            InstructionVariant::CallRoutine {
                                output_ty: expr.ty(),
                                nargs: opds.len().try_into().unwrap(),
                                linkage_fp: linkage,
                            },
                            expr.clone(),
                        )),
                        __Linkage::Model(_) => todo!(),
                    }
                } else {
                    self.push_instruction(Instruction::new(
                        InstructionVariant::CallInterpreted {
                            routine_uid: self.db.compile_time().entity_uid(routine.route),
                            nargs: opds.len().try_into().unwrap(),
                            has_this: false,
                            output_ty: expr.ty(),
                        },
                        expr.clone(),
                    ))
                }
            }
            EagerOpnVariant::Field {
                this_ty,
                field_ident,
                field_liason,
                field_binding,
                ..
            } => {
                self.push_instruction(Instruction::new(
                    if let Some(linkage) = self.db.compile_time().struct_field_access_linkage(
                        *this_ty,
                        field_ident.ident,
                        *field_binding,
                    ) {
                        InstructionVariant::CallRoutine {
                            linkage_fp: linkage,
                            nargs: 1,
                            output_ty: expr.ty(),
                        }
                    } else {
                        let this_ty_decl = self.db.compile_time().ty_decl(*this_ty).unwrap();
                        InstructionVariant::FieldAccessInterpreted {
                            field_idx: this_ty_decl
                                .field_idx(field_ident.ident)
                                .try_into()
                                .unwrap(),
                            field_binding: *field_binding,
                            field_ty: expr.ty(),
                        }
                    },
                    expr.clone(),
                ));
            }
            EagerOpnVariant::MethodCall {
                method_ident,
                this_ty_decl,
                method_route,
                output_binding: opt_output_binding,
            } => {
                let this = &opds[0];
                self.push_instruction(Instruction::new(
                    self.method_call_instruction_variant(
                        this.ty(),
                        this_ty_decl,
                        *method_route,
                        method_ident.ident,
                        expr.ty(),
                        *opt_output_binding,
                        opds.len().try_into().unwrap(),
                    ),
                    expr.clone(),
                ))
            }
            EagerOpnVariant::Index { element_binding } => {
                self.compile_element_access(expr.clone(), opds, *element_binding)
            }
            EagerOpnVariant::TypeCall {
                ranged_ty,
                ref ty_decl,
            } => {
                let ty_defn = self.db.compile_time().entity_defn(ranged_ty.route).unwrap();
                let instruction_variant = match ty_defn.variant {
                    EntityDefnVariant::Ty {
                        ty_kind: kind,
                        ref ty_members,
                        ..
                    } => {
                        self.context.enter();
                        self.context
                            .set(InstructionGenContext::NewVirtualStruct { output_stack_idx });
                        for (i, ty_member) in ty_members.iter().enumerate() {
                            match ty_member.variant {
                                EntityDefnVariant::TyField {
                                    ty,
                                    ref field_variant,
                                    liason,
                                    ..
                                } => match field_variant {
                                    FieldDefnVariant::StructOriginal => (),
                                    FieldDefnVariant::StructDefault { default } => {
                                        msg_once!("handle keyword arguments");
                                        self.compile_eager_expr(default, output_stack_idx + i)
                                    }
                                    FieldDefnVariant::StructDerivedEager { derivation } => {
                                        self.compile_eager_expr(derivation, output_stack_idx + i)
                                    }
                                    FieldDefnVariant::StructDerivedLazy { .. } => break,
                                    FieldDefnVariant::RecordOriginal
                                    | FieldDefnVariant::RecordDerived { .. } => panic!(),
                                },
                                _ => break,
                            }
                        }
                        self.context.exit();
                        if let Some(__Linkage) =
                            self.db.compile_time().type_call_linkage(ranged_ty.route)
                        {
                            match __Linkage {
                                __Linkage::Transfer(linkage) => InstructionVariant::CallRoutine {
                                    output_ty: expr.ty(),
                                    nargs: opds.len().try_into().unwrap(),
                                    linkage_fp: linkage,
                                },
                                __Linkage::Member(_) => todo!(),
                                __Linkage::Model(_) => todo!(),
                            }
                        } else {
                            match kind {
                                TyKind::Enum => todo!(),
                                TyKind::Record => todo!(),
                                TyKind::Struct => InstructionVariant::NewVirtualStruct {
                                    ty: ranged_ty.route,
                                    fields: ty_decl.eager_fields().map(|decl| decl.ident).collect(),
                                },
                                TyKind::Primitive => todo!(),
                                TyKind::Vec => todo!(),
                                TyKind::Array => todo!(),
                                TyKind::Other => todo!(),
                            }
                        }
                    }
                    _ => panic!(),
                };
                self.push_instruction(Instruction::new(instruction_variant, expr.clone()))
            }
            EagerOpnVariant::NewVecFromList => {
                let output_ty = expr.ty();
                let linkage = self.db.compile_time().type_call_linkage(output_ty).unwrap();
                self.push_instruction(Instruction::new(
                    match linkage {
                        __Linkage::Member(_) => todo!(),
                        __Linkage::Transfer(linkage) => InstructionVariant::CallRoutine {
                            linkage_fp: linkage,
                            nargs: opds.len().try_into().unwrap(),
                            output_ty,
                        },
                        __Linkage::Model(_) => todo!(),
                    },
                    expr.clone(),
                ))
            }
            EagerOpnVariant::ValueCall => self.push_instruction(Instruction::new(
                InstructionVariant::CallRoutine {
                    linkage_fp: __VALUE_CALL_LINKAGE.transfer(),
                    nargs: opds.len().try_into().unwrap(),
                    output_ty: expr.ty(),
                },
                expr.clone(),
            )),
        }
    }

    fn compile_binary_opn(
        &mut self,
        opr: BinaryOpr,
        this_ty: &EntityRoutePtr,
        opds: &[Arc<EagerExpr>],
        expr: &Arc<EagerExpr>,
    ) {
        match opds[0].ty() {
            EntityRoutePtr::Root(_) => {
                let ins_kind = InstructionVariant::CallRoutine {
                    linkage_fp: match opr {
                        BinaryOpr::Pure(pure_binary_opr) => {
                            resolve_primitive_pure_binary_opr_linkage(
                                opds[0].ty().root(),
                                pure_binary_opr,
                                opds[1].ty().root(),
                            )
                            .transfer()
                        }
                        BinaryOpr::Assign(opt_binary_opr) => {
                            resolve_primitive_assign_binary_opr_linkage(
                                opds[0].ty().root(),
                                opt_binary_opr,
                                opds[1].ty().root(),
                            )
                            .transfer()
                        }
                    },
                    nargs: 2,
                    output_ty: expr.ty(),
                };
                let instruction = Instruction::new(ins_kind, expr.clone());
                self.push_instruction(instruction)
            }
            EntityRoutePtr::Custom(_) => {
                let ins_kind = match opr {
                    BinaryOpr::Pure(pure_binary_opr) => match pure_binary_opr {
                        PureBinaryOpr::Add => todo!(),
                        PureBinaryOpr::And => todo!(),
                        PureBinaryOpr::BitAnd => todo!(),
                        PureBinaryOpr::BitOr => todo!(),
                        PureBinaryOpr::BitXor => todo!(),
                        PureBinaryOpr::Div => todo!(),
                        PureBinaryOpr::Eq => InstructionVariant::CallRoutine {
                            linkage_fp: __EQ_LINKAGE.transfer(),
                            nargs: 2,
                            output_ty: expr.ty(),
                        },
                        PureBinaryOpr::Geq => todo!(),
                        PureBinaryOpr::Greater => todo!(),
                        PureBinaryOpr::Leq => todo!(),
                        PureBinaryOpr::Less => todo!(),
                        PureBinaryOpr::Mul => todo!(),
                        PureBinaryOpr::Neq => InstructionVariant::CallRoutine {
                            linkage_fp: __NEQ_LINKAGE.transfer(),
                            nargs: 2,
                            output_ty: expr.ty(),
                        },
                        PureBinaryOpr::RemEuclid => todo!(),
                        PureBinaryOpr::Or => todo!(),
                        PureBinaryOpr::Power => todo!(),
                        PureBinaryOpr::Shl => todo!(),
                        PureBinaryOpr::Shr => todo!(),
                        PureBinaryOpr::Sub => todo!(),
                    },
                    BinaryOpr::Assign(opt_binary_opr) => todo!(),
                };
                let instruction = Instruction::new(ins_kind, expr.clone());
                self.push_instruction(instruction)
            }
            EntityRoutePtr::ThisType => todo!(),
        }
    }

    fn compile_element_access(
        &mut self,
        expr: Arc<EagerExpr>,
        opds: &[Arc<EagerExpr>],
        element_binding: Binding,
    ) {
        self.push_instruction(Instruction::new(
            InstructionVariant::CallRoutine {
                output_ty: expr.ty(),
                nargs: opds.len().try_into().unwrap(),
                linkage_fp: self
                    .db
                    .compile_time()
                    .element_access_linkage(opds.map(|opd| opd.ty()))
                    .bind(element_binding),
            },
            expr,
        ))
    }

    fn method_call_instruction_variant(
        &self,
        this_ty: EntityRoutePtr,
        this_ty_decl: &TyDecl,
        method_route: EntityRoutePtr,
        method_ident: CustomIdentifier,
        output_ty: EntityRoutePtr,
        output_binding: Binding,
        nargs: u8,
    ) -> InstructionVariant {
        if let Some(linkage) = self.db.compile_time().method_linkage(method_route) {
            match linkage {
                __Linkage::Member { .. } => InstructionVariant::CallRoutine {
                    linkage_fp: linkage.bind(output_binding),
                    nargs,
                    output_ty,
                },
                __Linkage::Transfer(linkage) => InstructionVariant::CallRoutine {
                    output_ty,
                    nargs,

                    linkage_fp: linkage,
                },
                __Linkage::Model(_) => todo!(),
            }
        } else {
            let method_uid = self.db.compile_time().entity_uid(method_route);
            let call_form_decl = self
                .db
                .compile_time()
                .entity_call_form_decl(method_route)
                .unwrap();
            InstructionVariant::CallInterpreted {
                routine_uid: method_uid,
                nargs: (call_form_decl.primary_parameters.len() + 1)
                    .try_into()
                    .unwrap(),
                has_this: true,
                output_ty,
            }
        }
    }
}
