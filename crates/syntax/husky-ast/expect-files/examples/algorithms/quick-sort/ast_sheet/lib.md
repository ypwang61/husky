Ok(
    AstSheet {
        arena: Arena {
            data: [
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    body: ArenaIdxRange(
                        2..2,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: ArenaIdxRange(
                        2..2,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: ArenaIdxRange(
                        2..2,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: ArenaIdxRange(
                        2..5,
                    ),
                },
                IfElseStmts {
                    if_stmt: 5,
                    elif_stmts: ArenaIdxRange(
                        6..6,
                    ),
                    else_stmt: None,
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    body: ArenaIdxRange(
                        7..7,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        18,
                    ),
                    body: ArenaIdxRange(
                        8..8,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    body: ArenaIdxRange(
                        9..9,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    body: ArenaIdxRange(
                        9..10,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        22,
                    ),
                    body: ArenaIdxRange(
                        11..11,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    body: ArenaIdxRange(
                        11..12,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    body: ArenaIdxRange(
                        7..7,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    body: ArenaIdxRange(
                        7..8,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: ArenaIdxRange(
                        8..8,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    body: ArenaIdxRange(
                        8..9,
                    ),
                },
                IfElseStmts {
                    if_stmt: 10,
                    elif_stmts: ArenaIdxRange(
                        11..11,
                    ),
                    else_stmt: Some(
                        12,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: ArenaIdxRange(
                        7..7,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: ArenaIdxRange(
                        7..7,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    body: ArenaIdxRange(
                        7..7,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    body: ArenaIdxRange(
                        13..18,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        23,
                    ),
                    body: ArenaIdxRange(
                        18..18,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        24,
                    ),
                    body: ArenaIdxRange(
                        18..18,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        27,
                    ),
                    body: ArenaIdxRange(
                        24..24,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        28,
                    ),
                    body: ArenaIdxRange(
                        24..24,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        29,
                    ),
                    body: ArenaIdxRange(
                        24..24,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        32,
                    ),
                    body: ArenaIdxRange(
                        27..27,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        33,
                    ),
                    body: ArenaIdxRange(
                        27..27,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        34,
                    ),
                    body: ArenaIdxRange(
                        27..27,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    body: ArenaIdxRange(
                        0..2,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Function,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        FormPath(`quick_sort::quick_sort, Function`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `quick_sort`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: true,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 3,
                    },
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        6..7,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 11,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Function,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        FormPath(`quick_sort::quick_sort_aux, Function`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `quick_sort_aux`,
                        token_idx: TokenIdx(
                            40,
                        ),
                    },
                    is_generic: true,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    body: ArenaIdxRange(
                        18..24,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 11,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Function,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        FormPath(`quick_sort::partition, Function`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `partition`,
                        token_idx: TokenIdx(
                            100,
                        ),
                    },
                    is_generic: true,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        25,
                    ),
                    body: ArenaIdxRange(
                        24..24,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        26,
                    ),
                    body: ArenaIdxRange(
                        24..27,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 11,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        FormPath(`quick_sort::quick_sort_works_for_integers, Feature`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `quick_sort_works_for_integers`,
                        token_idx: TokenIdx(
                            224,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        30,
                    ),
                    body: ArenaIdxRange(
                        27..27,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        31,
                    ),
                    body: ArenaIdxRange(
                        27..30,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 11,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        FormPath(`quick_sort::quick_sort_works_for_strs, Feature`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `quick_sort_works_for_strs`,
                        token_idx: TokenIdx(
                            284,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            30..37,
        ),
        use_expr_arena: Arena {
            data: [],
        },
    },
)