Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist::MnistLabel`, `Enum`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::Enum(
                            EnumTypeDefn {
                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                decl: EnumTypeDecl {
                                    path: TypePath(`mnist::MnistLabel`, `Enum`),
                                    ast_idx: 10,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist::BinaryImage28`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::TupleStruct(
                            TupleStructTypeDefn {
                                path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                decl: TupleStructTypeDecl {
                                    path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                    ast_idx: 11,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            28,
                                                        ),
                                                        Literal::Integer(
                                                            Unspecified,
                                                        ),
                                                    ),
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            27,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            29,
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 1,
                                                        argument: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            30,
                                                        ),
                                                        ident: `r32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: FieldType,
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            26,
                                        ),
                                    ),
                                    field_comma_list: (
                                        [
                                            TupleStructFieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ty: 3,
                                            },
                                        ],
                                        [],
                                    ),
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            31,
                                        ),
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)