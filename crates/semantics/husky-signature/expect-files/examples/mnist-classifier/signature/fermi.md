[
    Ok(
        Signature::Type(
            TypeSignature::RegularStruct(
                RegularStructTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: `matches`,
                            ty: RawTerm(`[] core::option::Option ~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                        },
                        RegularStructFieldSignature {
                            ident: `others`,
                            ty: RawTerm(`[] ~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Signature::Form(
            FormSignature::Fn(
                FnSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterSignatures {
                        parameters: [
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: ExplicitApplication(
                                    RawTermExplicitApplication(
                                        Id {
                                            value: 41,
                                        },
                                    ),
                                ),
                            },
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: ExplicitApplicationOrRitchieCall(
                                    RawTermExplicitApplicationOrRitchieCall(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: RawTerm(`mnist_classifier::fermi::FermiMatchResult`),
                },
            ),
        ),
    ),
    Ok(
        Signature::Impl(
            ImplSignature::TypeImpl(
                TypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    ty: RawTerm(`mnist_classifier::fermi::FermiMatchResult`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemSignature::TypeItem(
                TypeItemSignature::Memo(
                    TypeMemoSignature {
                        return_ty: RawTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemSignature::TypeItem(
                TypeItemSignature::Memo(
                    TypeMemoSignature {
                        return_ty: RawTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemSignature::TypeItem(
                TypeItemSignature::Memo(
                    TypeMemoSignature {
                        return_ty: RawTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
]