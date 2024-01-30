AstSheet {
    ast_arena: Arena {
        data: [
            Ast::Err {
                token_group_idx: TokenGroupIdx(
                    3,
                ),
                error: AstError::Original(
                    OriginalAstError::UnexpectedModInsideModuleItem,
                ),
            },
            Ast::Err {
                token_group_idx: TokenGroupIdx(
                    0,
                ),
                error: AstError::Original(
                    OriginalAstError::SubmoduleFileNotFound {
                        ident_token: IdentToken {
                            ident: `submodule_name`,
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                        error: VfsError::FileNotExists(
                            VirtualPath {
                                _data: VirtualPathBuf(
                                    "../../../examples/errors/syntax-errors/src/ast/submodule_name.hsy",
                                ),
                            },
                        ),
                    },
                ),
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    1,
                ),
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `syntax_errors::ast`,
                    ),
                },
                item_kind: MajorItem {
                    module_item_kind: Type(
                        Struct,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `A`,
                    token_idx: TokenIdx(
                        4,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        5,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`syntax_errors::ast::A`, `Struct`),
                    variants: None,
                },
            },
            Ast::ImplBlock {
                token_group_idx: TokenGroupIdx(
                    2,
                ),
                items: Some(
                    Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ),
                ),
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        2..5,
    ),
    siblings: [
        ArenaIdxRange(
            1..2,
        ),
        ArenaIdxRange(
            2..5,
        ),
    ],
}