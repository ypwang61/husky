Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::SubmoduleIdent,
            TokenInfo::None,
            TokenInfo::SubmoduleIdent,
            TokenInfo::None,
            TokenInfo::SubmoduleIdent,
            TokenInfo::None,
            TokenInfo::SubmoduleIdent,
            TokenInfo::None,
            TokenInfo::SubmoduleIdent,
            TokenInfo::None,
            TokenInfo::SubmoduleIdent,
            TokenInfo::None,
            TokenInfo::SubmoduleIdent,
            TokenInfo::None,
            TokenInfo::SubmoduleIdent,
            TokenInfo::None,
            TokenInfo::SubmoduleIdent,
            TokenInfo::None,
            TokenInfo::SubmoduleIdent,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 1,
                rule_idx: UseExprRuleIdx(
                    0,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::one`,
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `one`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 0,
                rule_idx: UseExprRuleIdx(
                    10,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 63,
                            ident_token: IdentToken {
                                ident: `is_one`,
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 4,
                rule_idx: UseExprRuleIdx(
                    1,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::SelfModule {
                        module_path: `mnist_classifier::digits`,
                    },
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 3,
                rule_idx: UseExprRuleIdx(
                    11,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::six`,
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `six`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 2,
                rule_idx: UseExprRuleIdx(
                    19,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 52,
                            ident_token: IdentToken {
                                ident: `is_six`,
                                token_idx: TokenIdx(
                                    37,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 7,
                rule_idx: UseExprRuleIdx(
                    2,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::SelfModule {
                        module_path: `mnist_classifier::digits`,
                    },
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 6,
                rule_idx: UseExprRuleIdx(
                    12,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::zero`,
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `zero`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 5,
                rule_idx: UseExprRuleIdx(
                    20,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 28,
                            ident_token: IdentToken {
                                ident: `is_zero`,
                                token_idx: TokenIdx(
                                    49,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 10,
                rule_idx: UseExprRuleIdx(
                    3,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::SelfModule {
                        module_path: `mnist_classifier::digits`,
                    },
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 9,
                rule_idx: UseExprRuleIdx(
                    13,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::two`,
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 9,
                            ident_token: IdentToken {
                                ident: `two`,
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 8,
                rule_idx: UseExprRuleIdx(
                    21,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 54,
                            ident_token: IdentToken {
                                ident: `is_two`,
                                token_idx: TokenIdx(
                                    113,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 13,
                rule_idx: UseExprRuleIdx(
                    4,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::SelfModule {
                        module_path: `mnist_classifier::digits`,
                    },
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 12,
                rule_idx: UseExprRuleIdx(
                    14,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::three`,
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `three`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 11,
                rule_idx: UseExprRuleIdx(
                    22,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 29,
                            ident_token: IdentToken {
                                ident: `is_three`,
                                token_idx: TokenIdx(
                                    26,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 16,
                rule_idx: UseExprRuleIdx(
                    5,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::SelfModule {
                        module_path: `mnist_classifier::digits`,
                    },
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 15,
                rule_idx: UseExprRuleIdx(
                    15,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::five`,
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 5,
                            ident_token: IdentToken {
                                ident: `five`,
                                token_idx: TokenIdx(
                                    11,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 14,
                rule_idx: UseExprRuleIdx(
                    23,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `is_five`,
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 19,
                rule_idx: UseExprRuleIdx(
                    6,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::SelfModule {
                        module_path: `mnist_classifier::digits`,
                    },
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 18,
                rule_idx: UseExprRuleIdx(
                    16,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::seven`,
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 6,
                            ident_token: IdentToken {
                                ident: `seven`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 17,
                rule_idx: UseExprRuleIdx(
                    24,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 45,
                            ident_token: IdentToken {
                                ident: `is_seven`,
                                token_idx: TokenIdx(
                                    164,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 22,
                rule_idx: UseExprRuleIdx(
                    7,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::SelfModule {
                        module_path: `mnist_classifier::digits`,
                    },
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 21,
                rule_idx: UseExprRuleIdx(
                    17,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::eight`,
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 7,
                            ident_token: IdentToken {
                                ident: `eight`,
                                token_idx: TokenIdx(
                                    15,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 20,
                rule_idx: UseExprRuleIdx(
                    25,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            ident_token: IdentToken {
                                ident: `is_eight`,
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 25,
                rule_idx: UseExprRuleIdx(
                    8,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::SelfModule {
                        module_path: `mnist_classifier::digits`,
                    },
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 24,
                rule_idx: UseExprRuleIdx(
                    18,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::nine`,
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 8,
                            ident_token: IdentToken {
                                ident: `nine`,
                                token_idx: TokenIdx(
                                    17,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 23,
                rule_idx: UseExprRuleIdx(
                    26,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 38,
                            ident_token: IdentToken {
                                ident: `is_nine`,
                                token_idx: TokenIdx(
                                    35,
                                ),
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 27,
                rule_idx: UseExprRuleIdx(
                    9,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: EntitySymbol::CrateRoot {
                        root_module_path: `mnist_classifier`,
                    },
                },
            },
            TokenInfo::None,
            TokenInfo::None,
        ],
    },
)