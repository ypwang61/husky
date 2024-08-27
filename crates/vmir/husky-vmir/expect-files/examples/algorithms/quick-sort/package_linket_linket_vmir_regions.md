```rust
[
    (
        Linket {
            data: LinketData::MajorFunctionRitchie {
                path: MajorFormPath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                    Fn,
                )`),
                instantiation: LinInstantiation {
                    path: ItemPath(`quick_sort::quick_sort_works_for_integers`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        Some(
            VmirRegion {
                linket: Linket {
                    data: LinketData::MajorFunctionRitchie {
                        path: MajorFormPath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                            Fn,
                        )`),
                        instantiation: LinInstantiation {
                            path: ItemPath(`quick_sort::quick_sort_works_for_integers`),
                            context: LinTypeContext {
                                comptime_var_overrides: [],
                            },
                            variable_resolutions: [],
                            separator: None,
                        },
                    },
                },
                root_expr: VmirExprIdx(
                    12,
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
                                0,
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
                        VmirExprData::Linket {
                            linket_impl: VirtualLinketImpl(
                                Linket {
                                    data: LinketData::VecConstructor {
                                        element_ty: LinType::PathLeading(
                                            LinTypePathLeading {
                                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                                template_arguments: [],
                                            },
                                        ),
                                    },
                                },
                            ),
                            arguments: [
                                VmirArgument::Variadic {
                                    exprs: VmirExprIdxRange(
                                        ArenaIdxRange(
                                            1..11,
                                        ),
                                    ),
                                },
                            ],
                        },
                        VmirExprData::Block {
                            stmts: VmirStmtIdxRange(
                                ArenaIdxRange(
                                    0..1,
                                ),
                            ),
                            destroyers: ArenaIdxRange(
                                0..0,
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
                                11,
                            ),
                            coercion: Some(
                                VmirCoercion::Trivial,
                            ),
                        },
                    ],
                },
            },
        ),
    ),
    (
        Linket {
            data: LinketData::MajorFunctionRitchie {
                path: MajorFormPath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                    Fn,
                )`),
                instantiation: LinInstantiation {
                    path: ItemPath(`quick_sort::quick_sort_works_for_strs`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        Some(
            VmirRegion {
                linket: Linket {
                    data: LinketData::MajorFunctionRitchie {
                        path: MajorFormPath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                            Fn,
                        )`),
                        instantiation: LinInstantiation {
                            path: ItemPath(`quick_sort::quick_sort_works_for_strs`),
                            context: LinTypeContext {
                                comptime_var_overrides: [],
                            },
                            variable_resolutions: [],
                            separator: None,
                        },
                    },
                },
                root_expr: VmirExprIdx(
                    7,
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
                        VmirExprData::Linket {
                            linket_impl: VirtualLinketImpl(
                                Linket {
                                    data: LinketData::VecConstructor {
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
                            ),
                            arguments: [
                                VmirArgument::Variadic {
                                    exprs: VmirExprIdxRange(
                                        ArenaIdxRange(
                                            0..6,
                                        ),
                                    ),
                                },
                            ],
                        },
                        VmirExprData::Block {
                            stmts: VmirStmtIdxRange(
                                ArenaIdxRange(
                                    0..1,
                                ),
                            ),
                            destroyers: ArenaIdxRange(
                                0..0,
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
                                6,
                            ),
                            coercion: None,
                        },
                    ],
                },
            },
        ),
    ),
    (
        Linket {
            data: LinketData::VecConstructor {
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
        Linket {
            data: LinketData::VecConstructor {
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