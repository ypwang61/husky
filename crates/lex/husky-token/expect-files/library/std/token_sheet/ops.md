Ok(
    TokenSheet {
        tokens: [
            Token {
                range: [1:1, 1:2),
                kind: Special(
                    PoundSign,
                ),
            },
            Token {
                range: [1:2, 1:3),
                kind: Special(
                    Bra(
                        Box,
                    ),
                ),
            },
            Token {
                range: [1:3, 1:10),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 33,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [1:11, 1:12),
                kind: Special(
                    BinaryOpr(
                        Assign(
                            None,
                        ),
                    ),
                ),
            },
            Token {
                range: [1:13, 1:28),
                kind: Literal(
                    String(
                        StringLiteral {
                            data: "std::ops::Add",
                        },
                    ),
                ),
            },
            Token {
                range: [1:28, 1:29),
                kind: Special(
                    Ket(
                        Box,
                    ),
                ),
            },
            Token {
                range: [2:1, 2:6),
                kind: Keyword(
                    Trait,
                ),
            },
            Token {
                range: [2:7, 2:10),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [2:10, 2:11),
                kind: Special(
                    LAngle,
                ),
            },
            Token {
                range: [2:11, 2:12),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 29,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [2:12, 2:13),
                kind: Special(
                    RAngle,
                ),
            },
            Token {
                range: [2:13, 2:14),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [3:5, 3:9),
                kind: Keyword(
                    Type(
                        Type,
                    ),
                ),
            },
            Token {
                range: [3:10, 3:16),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 30,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [3:16, 3:17),
                kind: Special(
                    Semicolon,
                ),
            },
            Token {
                range: [5:5, 5:7),
                kind: Keyword(
                    Paradigm(
                        Fn,
                    ),
                ),
            },
            Token {
                range: [5:8, 5:11),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 24,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [5:11, 5:12),
                kind: Special(
                    Bra(
                        Par,
                    ),
                ),
            },
            Token {
                range: [5:12, 5:17),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [5:17, 5:18),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [5:19, 5:20),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 29,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [5:20, 5:21),
                kind: Special(
                    Ket(
                        Par,
                    ),
                ),
            },
            Token {
                range: [5:22, 5:24),
                kind: Special(
                    BinaryOpr(
                        Curry,
                    ),
                ),
            },
            Token {
                range: [5:25, 5:29),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 31,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [5:29, 5:31),
                kind: Special(
                    BinaryOpr(
                        ScopeResolution,
                    ),
                ),
            },
            Token {
                range: [5:31, 5:37),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 30,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [5:37, 5:38),
                kind: Special(
                    Semicolon,
                ),
            },
        ],
        group_starts: [
            0,
            6,
            12,
            15,
        ],
    },
)