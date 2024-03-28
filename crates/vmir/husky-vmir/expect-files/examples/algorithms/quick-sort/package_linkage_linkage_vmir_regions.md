```rust
[
    (
        Linkage {
            data: LinkageData::MajorRitchieEager {
                path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                    Fn,
                )`),
                instantiation: LinInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
        Some(
            VmirRegion {
                linkage: Linkage {
                    data: LinkageData::MajorRitchieEager {
                        path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                            Fn,
                        )`),
                        instantiation: LinInstantiation {
                            symbol_resolutions: [],
                            separator: None,
                        },
                    },
                },
                root_expr: VmirExprIdx(
                    13,
                ),
                vmir_expr_arena: Arena {
                    data: [
                        VmirExprData::Literal {
                            value: I32(
                                31,
                            ),
                        },
                        VmirExprData::Literal {
                            value: I32(
                                4,
                            ),
                        },
                        VmirExprData::Literal {
                            value: I32(
                                65,
                            ),
                        },
                        VmirExprData::Literal {
                            value: I32(
                                2,
                            ),
                        },
                        VmirExprData::Prefix {
                            opr: Minus,
                            opd: VmirExprIdx(
                                1,
                            ),
                        },
                        VmirExprData::Literal {
                            value: I32(
                                0,
                            ),
                        },
                        VmirExprData::Literal {
                            value: I32(
                                99,
                            ),
                        },
                        VmirExprData::Literal {
                            value: I32(
                                2,
                            ),
                        },
                        VmirExprData::Literal {
                            value: I32(
                                83,
                            ),
                        },
                        VmirExprData::Literal {
                            value: I32(
                                782,
                            ),
                        },
                        VmirExprData::Literal {
                            value: I32(
                                1,
                            ),
                        },
                        VmirExprData::Linkage {
                            linkage_impl: Linkage {
                                data: LinkageData::VecConstructor {
                                    element_ty: LinType::PathLeading(
                                        LinTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                        },
                                    ),
                                },
                            },
                            arguments: [
                                VmirArgument::Variadic {
                                    exprs: ArenaIdxRange(
                                        2..12,
                                    ),
                                },
                            ],
                        },
                        VmirExprData::Block {
                            stmts: VmirStmtIdxRange(
                                ArenaIdxRange(
                                    1..2,
                                ),
                            ),
                            destroyers: ArenaIdxRange(
                                1..1,
                            ),
                        },
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [
                        VmirStmtData::Let {
                            pattern: VmirPattern {
                                restructive_pattern: VmirRestructivePattern::Default(
                                    None,
                                ),
                                destructive_pattern: Some(
                                    VmirDestructivePattern::Default(
                                        None,
                                    ),
                                ),
                            },
                            initial_value: VmirExprIdx(
                                12,
                            ),
                            coersion: Some(
                                VmirCoersion::Trivial,
                            ),
                        },
                    ],
                },
            },
        ),
    ),
    (
        Linkage {
            data: LinkageData::MajorRitchieEager {
                path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                    Fn,
                )`),
                instantiation: LinInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
        Some(
            VmirRegion {
                linkage: Linkage {
                    data: LinkageData::MajorRitchieEager {
                        path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                            Fn,
                        )`),
                        instantiation: LinInstantiation {
                            symbol_resolutions: [],
                            separator: None,
                        },
                    },
                },
                root_expr: VmirExprIdx(
                    8,
                ),
                vmir_expr_arena: Arena {
                    data: [
                        VmirExprData::Literal {
                            value: String(
                                "beach",
                            ),
                        },
                        VmirExprData::Literal {
                            value: String(
                                "hotel",
                            ),
                        },
                        VmirExprData::Literal {
                            value: String(
                                "airplane",
                            ),
                        },
                        VmirExprData::Literal {
                            value: String(
                                "car",
                            ),
                        },
                        VmirExprData::Literal {
                            value: String(
                                "house",
                            ),
                        },
                        VmirExprData::Literal {
                            value: String(
                                "art",
                            ),
                        },
                        VmirExprData::Linkage {
                            linkage_impl: Linkage {
                                data: LinkageData::VecConstructor {
                                    element_ty: LinType::PathLeading(
                                        LinTypePathLeading {
                                            ty_path: TypePath(`core::mem::Ref`, `Extern`),
                                            template_arguments: [
                                                LinTemplateArgument::Constant(
                                                    LinConstant(
                                                        StaticLifetime,
                                                    ),
                                                ),
                                                LinTemplateArgument::Type(
                                                    LinType::PathLeading(
                                                        LinTypePathLeading {
                                                            ty_path: TypePath(`core::str::str`, `Extern`),
                                                            template_arguments: [],
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                    ),
                                },
                            },
                            arguments: [
                                VmirArgument::Variadic {
                                    exprs: ArenaIdxRange(
                                        1..7,
                                    ),
                                },
                            ],
                        },
                        VmirExprData::Block {
                            stmts: VmirStmtIdxRange(
                                ArenaIdxRange(
                                    1..2,
                                ),
                            ),
                            destroyers: ArenaIdxRange(
                                1..1,
                            ),
                        },
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [
                        VmirStmtData::Let {
                            pattern: VmirPattern {
                                restructive_pattern: VmirRestructivePattern::Default(
                                    None,
                                ),
                                destructive_pattern: Some(
                                    VmirDestructivePattern::Default(
                                        None,
                                    ),
                                ),
                            },
                            initial_value: VmirExprIdx(
                                7,
                            ),
                            coersion: None,
                        },
                    ],
                },
            },
        ),
    ),
    (
        Linkage {
            data: LinkageData::VecConstructor {
                element_ty: LinType::PathLeading(
                    LinTypePathLeading {
                        ty_path: TypePath(`core::num::i32`, `Extern`),
                        template_arguments: [],
                    },
                ),
            },
        },
        None,
    ),
    (
        Linkage {
            data: LinkageData::VecConstructor {
                element_ty: LinType::PathLeading(
                    LinTypePathLeading {
                        ty_path: TypePath(`core::mem::Ref`, `Extern`),
                        template_arguments: [
                            LinTemplateArgument::Constant(
                                LinConstant(
                                    StaticLifetime,
                                ),
                            ),
                            LinTemplateArgument::Type(
                                LinType::PathLeading(
                                    LinTypePathLeading {
                                        ty_path: TypePath(`core::str::str`, `Extern`),
                                        template_arguments: [],
                                    },
                                ),
                            ),
                        ],
                    },
                ),
            },
        },
        None,
    ),
]
```