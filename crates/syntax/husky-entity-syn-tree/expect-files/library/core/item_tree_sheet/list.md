Ok(
    EntitySynTreeSheet {
        module_path: `core::list`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: ItemSynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::list::List`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 6,
                            ident_token: IdentToken {
                                ident: `List`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 7,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::ModuleItem(
                        ModuleItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::list::List`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `List`,
                    visibility: Scope::Pub,
                },
            ],
        },
        item_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `List`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Type(
                            TypePath(`core::list::List`, `Extern`),
                        ),
                        node: ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::list::List`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 6,
                            ident_token: IdentToken {
                                ident: `List`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 7,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
            ],
        ),
        impl_block_syn_node_table: [
            (
                ImplBlockSynNodePath::TypeImplBlock(
                    TypeImplBlockSynNodePath {
                        path: TypeImplBlockPath {
                            module_path: `core::list`,
                            ty_path: TypePath(`core::list::List`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockSynNode::TypeImplBlock(
                    TypeImplBlockSynNode {
                        syn_node_path: TypeImplBlockSynNodePath {
                            path: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 7,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                8,
                            ),
                        },
                        ty_expr: 1,
                        items: TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                0..6,
                            ),
                        },
                    },
                ),
            ),
        ],
        once_use_rules: OnceUseRules(
            [],
        ),
        use_all_rules: UseAllModuleSymbolsRules(
            [],
        ),
        errors: [],
    },
)