```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [
                    ValkyrieRide::VecConstructor {
                        element_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                    },
                    ValkyrieRide::PathLeading {
                        path: JavPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                `<#derive _ as core::clone::Clone(0)>::clone`,
                                TraitItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::SelfType,
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::vec::Vec(0)::push`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::SelfLifetime,
                                    ),
                                    HirTermSymbolicVariableResolution::SelfLifetime,
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                ],
            },
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    `<mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)>::visualize`,
                    TraitItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::vec::Vec(0)::collect_leashes`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide::VecConstructor {
                        element_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                template_arguments: [
                                                    HirTemplateArgument::Type(
                                                        HirType::PathLeading(
                                                            HirTypePathLeading {
                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                template_arguments: [],
                                                                always_copyable: false,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                                always_copyable: true,
                                            },
                                        ),
                                    ),
                                ],
                                always_copyable: true,
                            },
                        ),
                    },
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::vec::Vec(0)::pop_with_largest_opt_f32`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::SelfLifetime,
                                    ),
                                    HirTermSymbolicVariableResolution::SelfLifetime,
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::vec::Vec(0)::push`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::option::Option`, `Enum`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                                    template_arguments: [
                                                                        HirTemplateArgument::Type(
                                                                            HirType::PathLeading(
                                                                                HirTypePathLeading {
                                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                                    template_arguments: [],
                                                                                    always_copyable: false,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ],
                                                                    always_copyable: true,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::SelfLifetime,
                                    ),
                                    HirTermSymbolicVariableResolution::SelfLifetime,
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                ],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::vec::Vec(0)::ilen`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide::PathLeading {
                        path: JavPath::TypeItem(
                            TypeItemPath(
                                `core::vec::Vec(0)::ilen`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVariable::Type(
                                        HirTypeTemplateVariable::Type {
                                            attrs: HirTemplateVariableAttrs {
                                                class: Mono,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolicVariableResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                ],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::row_span_sum`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::distribution`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`,
                    TypeItemKind::MemoizedField,
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`,
                    TypeItemKind::MethodRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
    ),
]
```