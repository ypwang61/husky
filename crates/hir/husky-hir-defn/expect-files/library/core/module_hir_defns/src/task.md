```rust
[
    HirDefn::MajorItem(
        MajorItemHirDefn::Trait(
            TraitHirDefn {
                path: TraitPath(`core::task::IsTask`),
                hir_decl: TraitHirDecl {
                    path: TraitPath(`core::task::IsTask`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::MajorItem(
                                MajorItemPath::Trait(
                                    TraitPath(`core::task::IsTask`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            },
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Form(
            MajorFormHirDefn::StaticVar(
                MajorStaticVarHirDefn {
                    path: FormPath(`core::task::TASK`, `StaticVar`),
                    hir_decl: MajorStaticVarHirDecl {
                        path: FormPath(`core::task::TASK`, `StaticVar`),
                        return_ty: HirType::TypeVar(
                            FormPath(`core::task::Task`, `TypeVar`),
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`core::task::TASK`, `StaticVar`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    hir_expr_body_and_region: None,
                },
            ),
        ),
    ),
]
```