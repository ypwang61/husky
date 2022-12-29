Ok(
    AstSheet {
        arena: Arena {
            data: [
                Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Foreign,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath(`core::basic::bool, Foreign`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `bool`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIterState {
                        next_relative: 3,
                    },
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Structure,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath(`core::basic::Trait, Structure`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `Trait`,
                        token_idx: TokenIdx(
                            6,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIterState {
                        next_relative: 3,
                    },
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Structure,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath(`core::basic::Module, Structure`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `Module`,
                        token_idx: TokenIdx(
                            10,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIterState {
                        next_relative: 3,
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..3,
        ),
        use_expr_arena: Arena {
            data: [],
        },
    },
)