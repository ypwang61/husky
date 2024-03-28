```rust
[
    (
        FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
        None,
    ),
    (
        FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
        Some(
            KiReprExpansion {
                hir_lazy_variable_ki_repr_map: ArenaMap {
                    data: [
                        Some(
                            KiRepr(
                                Id {
                                    value: 23,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 53,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 57,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 61,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 92,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 140,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 142,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 197,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 199,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 208,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 210,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 221,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 223,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 225,
                                },
                            ),
                        ),
                        Some(
                            KiRepr(
                                Id {
                                    value: 227,
                                },
                            ),
                        ),
                    ],
                },
                hir_lazy_expr_ki_repr_map: [
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_hole_ilen`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 3,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Literal(
                            Literal::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 5,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieLazy {
                                    path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    KiRepr(
                                        Id {
                                            value: 15,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 16,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 17,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 6,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocFn {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 18,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 7,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::VecConstructor {
                                    element_ty: LinType::Ritchie(
                                        LinkageRitchieType {
                                            parameters: [
                                                LinkageRitchieParameter {
                                                    contract: Pure,
                                                    parameter_ty: PathLeading(
                                                        LinTypePathLeading(
                                                            Id {
                                                                value: 3,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ],
                                            return_ty: LinType::PathLeading(
                                                LinTypePathLeading {
                                                    ty_path: TypePath(`core::option::Option`, `Enum`),
                                                    template_arguments: [
                                                        LinTemplateArgument::Type(
                                                            LinType::PathLeading(
                                                                LinTypePathLeading {
                                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                                    template_arguments: [],
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                },
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 10,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieEager {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                        Fn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 11,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieEager {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                        Fn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 43,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 13,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        3.0,
                                    ),
                                    text: "3.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 14,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 24,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 25,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 15,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_row_span`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 18,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 19,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieLazy {
                                    path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    KiRepr(
                                        Id {
                                            value: 27,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 28,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 20,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocFn {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 29,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 21,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 30,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_row_span`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 23,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 30,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        6.5,
                                    ),
                                    text: "6.5f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 24,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 30,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Greater,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 31,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 32,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 25,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 33,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_hole_ilen`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 27,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 33,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        0.0,
                                    ),
                                    text: "0.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 28,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 33,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 35,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 36,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 29,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 39,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 252,
                                    },
                                ),
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 30,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: ConditionNotSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_hole_ilen`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 32,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionNotSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        0.0,
                                    ),
                                    text: "0.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 33,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionNotSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 43,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 34,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 45,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        0.0,
                                    ),
                                    text: "0.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 36,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 45,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 37,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 39,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 40,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 41,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 43,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 1,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 44,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 45,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 47,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 2,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 48,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 58,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 59,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 49,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieEager {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                        Fn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 43,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 62,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 52,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 62,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        4.2,
                                    ),
                                    text: "4.2f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 53,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 62,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 64,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 65,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 54,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieEager {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                        Fn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 43,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::angle_change_norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 57,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<core::num::f32(0)>::abs`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 68,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 58,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 59,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieLazy {
                                    path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    KiRepr(
                                        Id {
                                            value: 69,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 70,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 60,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 67,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocFn {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 71,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 61,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        2.0,
                                    ),
                                    text: "2.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 62,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::lower_mass`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 64,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Mul,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 74,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 75,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 65,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::upper_mass`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 67,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 76,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 77,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 68,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        52.0,
                                    ),
                                    text: "52.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 69,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 78,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 79,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 70,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 81,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 252,
                                    },
                                ),
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 71,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 73,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 85,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 74,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<core::slice::CyclicSlice(0)>::end`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 8,
                                                                },
                                                            ),
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
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 86,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 75,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 77,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 88,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 78,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<core::slice::CyclicSlice(0)>::start`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 8,
                                                                },
                                                            ),
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
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 89,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 79,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 87,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 90,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 80,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 82,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        1.0,
                                    ),
                                    text: "1.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 83,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 94,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 95,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 84,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                3,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 87,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::top_k_row_span_sum`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 98,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 88,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 90,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::rel_norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 92,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::fermi::FermiMatchResult(0)>::angle_change_norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 94,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                12,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 95,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieLazy {
                                    path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    KiRepr(
                                        Id {
                                            value: 99,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 100,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 101,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 102,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 103,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 96,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 97,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocFn {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 104,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 97,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 100,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 101,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 106,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 107,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 102,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 108,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 103,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::rel_norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 109,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 104,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 106,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 107,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 111,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 112,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 108,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 113,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 109,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::angle_change`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 114,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 110,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<core::num::f32(0)>::abs`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 115,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 111,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 112,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieLazy {
                                    path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    KiRepr(
                                        Id {
                                            value: 110,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 116,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 117,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 113,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 105,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocFn {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 118,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 114,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 117,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 122,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 118,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<core::slice::CyclicSlice(0)>::end`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 8,
                                                                },
                                                            ),
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
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 123,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 119,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 121,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 125,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 122,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<core::slice::CyclicSlice(0)>::start`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 8,
                                                                },
                                                            ),
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
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 126,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 123,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 124,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 127,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 124,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 125,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 128,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 129,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 126,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 128,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 132,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 129,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 131,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 134,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 132,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<core::slice::CyclicSlice(0)>::start`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 8,
                                                                },
                                                            ),
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
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 135,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 133,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                1,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 134,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Add,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 136,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 137,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 135,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 133,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 138,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 136,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 133,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 138,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 10,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke(0)>::displacement`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 138,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke(0)>::displacement`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 11,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 89,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 140,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        0.0,
                                    ),
                                    text: "0.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 141,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Greater,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 144,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 145,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 142,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 145,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 148,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 146,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 148,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::rel_norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 150,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 149,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 19,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 151,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::angle_change`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 152,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 152,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<core::num::f32(0)>::abs`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 153,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 153,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 154,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieLazy {
                                    path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    KiRepr(
                                        Id {
                                            value: 149,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 151,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 154,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 155,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 155,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocFn {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 156,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 156,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke(0)>::displacement`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 11,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::geom2d::Vector2d(0)>::norm`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 159,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke(0)>::displacement`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 11,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 88,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 161,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke(0)>::displacement`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 11,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 89,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 163,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Div,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 159,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 160,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 164,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 165,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieLazy {
                                    path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    KiRepr(
                                        Id {
                                            value: 158,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 161,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 162,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 166,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocFn {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 163,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 167,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 58,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 59,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 20,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 171,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 2,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 172,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 167,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 168,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 173,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 169,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 174,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 170,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 175,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 177,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 2,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 178,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 172,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 173,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 179,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 174,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 180,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::rel_norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 175,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 181,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 183,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                USizeLiteral {
                                    value: 2,
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 184,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 177,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 178,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 185,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 179,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 186,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::angle_change`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 180,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 187,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<core::num::f32(0)>::abs`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 181,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 188,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 189,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieLazy {
                                    path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    KiRepr(
                                        Id {
                                            value: 171,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 176,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 182,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 183,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 190,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 166,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocFn {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 184,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 191,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 87,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 90,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 22,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 186,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                3,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 193,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 186,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 92,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 188,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 194,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 196,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 191,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 197,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 199,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 193,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 200,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<core::slice::CyclicSlice(0)>::start`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 8,
                                                                },
                                                            ),
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
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 194,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 201,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 192,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 195,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 202,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 192,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 195,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 29,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke(0)>::displacement`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 197,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 204,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 206,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 200,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 207,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 18,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 209,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            LinTemplateArgument::Type(
                                                LinType::PathLeading(
                                                    LinTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 216,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 202,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 210,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<core::slice::CyclicSlice(0)>::start`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 8,
                                                                },
                                                            ),
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
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 203,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 211,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                2,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 212,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Add,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 204,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 205,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 213,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 201,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 206,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 214,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 201,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 206,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 31,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke(0)>::displacement`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 208,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 216,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke(0)>::displacement`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 197,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 30,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 89,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 199,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 219,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodRitchie {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke(0)>::displacement`, `MethodRitchie(
                                            Fn,
                                        )`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 208,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 32,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 89,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 210,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 221,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 222,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorRitchieLazy {
                                    path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSvarAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    KiRepr(
                                        Id {
                                            value: 211,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 212,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    KiRepr(
                                        Id {
                                            value: 213,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    KiRuntimeConstant(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 223,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 190,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocFn {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 214,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 224,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 87,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 90,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 22,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 215,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                3,
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 226,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 215,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 92,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 217,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 227,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::lower_mass`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 229,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: KiReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::upper_mass`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 231,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::lower_mass`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 35,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::upper_mass`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 36,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 223,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 234,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 223,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 37,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::lower_mass`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 35,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Add,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 225,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 237,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 223,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 37,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        10.0,
                                    ),
                                    text: "10.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 239,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Prefix(
                            Minus,
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 229,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 240,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Geq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 225,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 230,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 241,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 223,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 37,
                            },
                        },
                        caching_class: Variable,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 232,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                F32Literal {
                                    value: OrderedFloat(
                                        20.0,
                                    ),
                                    text: "20.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 243,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 232,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Leq,
                            ),
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 225,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 234,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 244,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 236,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId(
                                    Id {
                                        value: 252,
                                    },
                                ),
                            ),
                        ),
                        arguments: [],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 245,
                            },
                        },
                        caching_class: Expr,
                    },
                ],
                hir_lazy_stmt_ki_repr_map: [
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 33,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 37,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 34,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 1,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 30,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 33,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 38,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 3,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 62,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 66,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 63,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 5,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 72,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 80,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 73,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 7,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 81,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 82,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 8,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: ConditionSatisfied(
                            KiRepr(
                                Id {
                                    value: 120,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 130,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 121,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 9,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 146,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 143,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 12,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: ConditionNotSatisfied(
                            KiRepr(
                                Id {
                                    value: 26,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 41,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 16,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 45,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 46,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 17,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 62,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 67,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 72,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 81,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 83,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 21,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 84,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 96,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 93,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 23,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 119,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 120,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 131,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 147,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 157,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 164,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 26,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 165,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 166,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 185,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 27,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 186,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 189,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 187,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 28,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 215,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 218,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 216,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 34,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 219,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 231,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 228,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 39,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 232,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 235,
                                    },
                                ),
                            ),
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 233,
                                    },
                                ),
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 40,
                            },
                        },
                        caching_class: Stmt,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 26,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 30,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ],
                            },
                            Branch {
                                condition: None,
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 45,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 49,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 84,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 97,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 105,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 119,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 165,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 186,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 190,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 215,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 219,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 232,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 236,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 237,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 44,
                            },
                        },
                        caching_class: Stmt,
                    },
                ],
                root_hir_lazy_stmt_ki_reprs: [
                    KiRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocFn {
                                    path: TraitForTypeItemPath(
                                        `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                        TraitItemKind::AssocRitchie(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Simple(
                                KiRepr(
                                    Id {
                                        value: 18,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 7,
                            },
                        },
                        caching_class: Expr,
                    },
                    KiRepr {
                        val_domain_repr: StmtNotReturned(
                            KiRepr(
                                Id {
                                    value: 19,
                                },
                            ),
                        ),
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    KiRepr(
                                        Id {
                                            value: 26,
                                        },
                                    ),
                                ),
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 30,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ],
                            },
                            Branch {
                                condition: None,
                                stmts: [
                                    KiRepr(
                                        Id {
                                            value: 45,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 49,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 84,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 97,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 105,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 119,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 165,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 186,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 190,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 215,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 219,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 232,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 236,
                                        },
                                    ),
                                    KiRepr(
                                        Id {
                                            value: 237,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: KiReprSource::Expansion {
                            parent_ki_repr: KiRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                arguments: [],
                                source: KiReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 44,
                            },
                        },
                        caching_class: Stmt,
                    },
                ],
            },
        ),
    ),
]
```