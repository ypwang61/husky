```rust
[
    (
        TokenIdx(
            1,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Keyword(\n    Keyword::Use,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 0,
                            },
                            end: Position {
                                line: 0,
                                character: 3,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            45,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "This is a paradigm\ntoken_idx = 44;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Keyword(\n    Keyword::Form(\n        Fn,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 14,
                                character: 0,
                            },
                            end: Position {
                                line: 14,
                                character: 2,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            89,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 88;\n\ntoken_line_group_idx = 10\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 25,
                                character: 28,
                            },
                            end: Position {
                                line: 25,
                                character: 29,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            133,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 132;\n\ntoken_line_group_idx = 17\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Question,\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                6,\n            ),\n        ),\n        data: TokenInfoData::SemaPrefixTypeOpr,\n    },\n);\n\nSemaPrefixTypeOpr\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 34,
                                character: 27,
                            },
                            end: Position {
                                line: 34,
                                character: 28,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            177,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 176;\n\ntoken_line_group_idx = 22\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 41,
                                character: 28,
                            },
                            end: Position {
                                line: 41,
                                character: 29,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            221,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 220;\n\ntoken_line_group_idx = 29\n\ntoken = TokenData::Keyword(\n    Keyword::Stmt(\n        Return,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 49,
                                character: 8,
                            },
                            end: Position {
                                line: 49,
                                character: 14,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            265,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "This is a paradigm\ntoken_idx = 264;\n\ntoken_line_group_idx = 35\n\ntoken = TokenData::Keyword(\n    Keyword::Form(\n        Memo,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 58,
                                character: 4,
                            },
                            end: Position {
                                line: 58,
                                character: 8,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            309,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 308;\n\ntoken_line_group_idx = 42\n\ntoken = TokenData::Ident(\n    `row_start`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                1,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_variable_idx: 0,\n            current_variable_kind: CurrentVariableKind::LetVariable {\n                pattern_variable_idx: 0,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        4,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                118,\n            ),\n        ),\n    ),\n    data: CurrentVariableData::LetVariable {\n        ident: `row_start`,\n        pattern_variable_idx: 0,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 67,
                                character: 15,
                            },
                            end: Position {
                                line: 67,
                                character: 24,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            353,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 352;\n\ntoken_line_group_idx = 50\n\ntoken = TokenData::Ident(\n    `height`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                20,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_variable_idx: 2,\n            current_variable_kind: CurrentVariableKind::LetVariable {\n                pattern_variable_idx: 2,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Pure,\n    access_start: RegionalTokenIdx(\n        44,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                118,\n            ),\n        ),\n    ),\n    data: CurrentVariableData::LetVariable {\n        ident: `height`,\n        pattern_variable_idx: 2,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 75,
                                character: 26,
                            },
                            end: Position {
                                line: 75,
                                character: 32,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            397,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 396;\n\ntoken_line_group_idx = 56\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            SynBinaryOpr::AssignClosed(\n                BinaryClosedOpr::Add,\n            ),\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 81,
                                character: 23,
                            },
                            end: Position {
                                line: 81,
                                character: 25,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            441,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 440;\n\ntoken_line_group_idx = 61\n\ntoken = TokenData::Ident(\n    `lower_mass`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                3,\n            ),\n        ),\n        data: TokenInfoData::Field,\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 93,
                                character: 26,
                            },
                            end: Position {
                                line: 93,
                                character: 36,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            485,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 484;\n\ntoken_line_group_idx = 69\n\ntoken = TokenData::Ident(\n    `j`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                15,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_variable_idx: 2,\n            current_variable_kind: CurrentVariableKind::LoopVariable(\n                13,\n            ),\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Pure,\n    access_start: RegionalTokenIdx(\n        38,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                50,\n            ),\n        ),\n    ),\n    data: CurrentVariableData::LoopVariable {\n        ident: `j`,\n        expr_idx: 13,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 102,
                                character: 17,
                            },
                            end: Position {
                                line: 102,
                                character: 18,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            529,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 528;\n\ntoken_line_group_idx = 75\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 109,
                                character: 18,
                            },
                            end: Position {
                                line: 109,
                                character: 19,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            573,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 572;\n\ntoken_line_group_idx = 82\n\ntoken = TokenData::Ident(\n    `a`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::Pattern(\n            0,\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_variable_idx: 0,\n            current_variable_kind: CurrentVariableKind::SimpleParenateParameter {\n                pattern_variable_idx: 0,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Pure,\n    access_start: RegionalTokenIdx(\n        5,\n    ),\n    access_end: None,\n    data: CurrentVariableData::SimpleParenateParameter {\n        ident: `a`,\n        pattern_variable_idx: 0,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 117,
                                character: 21,
                            },
                            end: Position {
                                line: 117,
                                character: 22,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            617,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 616;\n\ntoken_line_group_idx = 84\n\ntoken = TokenData::Literal(\n    LiteralTokenData::Integer(\n        UnspecifiedRegular(\n            1,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 119,
                                character: 31,
                            },
                            end: Position {
                                line: 119,
                                character: 32,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            661,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 660;\n\ntoken_line_group_idx = 89\n\ntoken = TokenData::Ident(\n    `BinaryImage28`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            0,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`mnist::BinaryImage28`, `Extern`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`mnist::BinaryImage28`, `Extern`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 125,
                                character: 38,
                            },
                            end: Position {
                                line: 125,
                                character: 51,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            705,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 704;\n\ntoken_line_group_idx = 95\n\ntoken = TokenData::Keyword(\n    Keyword::Stmt(\n        Let,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 131,
                                character: 12,
                            },
                            end: Position {
                                line: 131,
                                character: 15,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            749,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 748;\n\ntoken_line_group_idx = 101\n\ntoken = TokenData::Ident(\n    `i`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::Pattern(\n            6,\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_variable_idx: 7,\n            current_variable_kind: CurrentVariableKind::LetVariable {\n                pattern_variable_idx: 6,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        83,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                186,\n            ),\n        ),\n    ),\n    data: CurrentVariableData::LetVariable {\n        ident: `i`,\n        pattern_variable_idx: 6,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 137,
                                character: 24,
                            },
                            end: Position {
                                line: 137,
                                character: 25,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            793,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 792;\n\ntoken_line_group_idx = 107\n\ntoken = TokenData::Ident(\n    `old_row`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                58,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_variable_idx: 8,\n            current_variable_kind: CurrentVariableKind::LetVariable {\n                pattern_variable_idx: 7,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Pure,\n    access_start: RegionalTokenIdx(\n        94,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                141,\n            ),\n        ),\n    ),\n    data: CurrentVariableData::LetVariable {\n        ident: `old_row`,\n        pattern_variable_idx: 7,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 143,
                                character: 23,
                            },
                            end: Position {
                                line: 143,
                                character: 30,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            837,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 836;\n\ntoken_line_group_idx = 112\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::RightDelimiter(\n            Delimiter::Box,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 148,
                                character: 80,
                            },
                            end: Position {
                                line: 148,
                                character: 81,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            881,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 880;\n\ntoken_line_group_idx = 119\n\ntoken = TokenData::Keyword(\n    Keyword::Stmt(\n        Return,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 155,
                                character: 4,
                            },
                            end: Position {
                                line: 155,
                                character: 10,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
]
```