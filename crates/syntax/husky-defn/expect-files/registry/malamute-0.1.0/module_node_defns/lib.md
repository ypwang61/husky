Ok(
    [
        NodeDefn::ModuleItem(
            ModuleItemNodeDefn::Type(
                TypeNodeDefn::Enum(
                    EnumTypeNodeDefn {
                        node_path: TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`malamute::OneVsAll`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                        node_decl: EnumTypeNodeDecl {
                            node_path: TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 5,
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                3,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: ImplicitParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `Label`,
                                                        token_idx: TokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            ImplicitParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: ImplicitParameterDeclPatternVariant::Constant {
                                                    const_token: ConstToken {
                                                        token_idx: TokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    ident_token: IdentToken {
                                                        ident: `label`,
                                                        token_idx: TokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                    colon_token: ColonToken(
                                                        TokenIdx(
                                                            8,
                                                        ),
                                                    ),
                                                    ty_expr: 0,
                                                },
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    5,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                10,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Type(
                                                TypeNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    9,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
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
                                            data: [
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        10,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                            ident_token: IdentToken {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    7,
                                                                ),
                                                            },
                                                            ty_expr_idx: 0,
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ConstantImplicitParameterType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        NodeDefn::ModuleItem(
            ModuleItemNodeDefn::Type(
                TypeNodeDefn::Enum(
                    EnumTypeNodeDefn {
                        node_path: TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                        node_decl: EnumTypeNodeDecl {
                            node_path: TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 6,
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                18,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: ImplicitParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `Label`,
                                                        token_idx: TokenIdx(
                                                            19,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            ImplicitParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: ImplicitParameterDeclPatternVariant::Constant {
                                                    const_token: ConstToken {
                                                        token_idx: TokenIdx(
                                                            21,
                                                        ),
                                                    },
                                                    ident_token: IdentToken {
                                                        ident: `label`,
                                                        token_idx: TokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                    colon_token: ColonToken(
                                                        TokenIdx(
                                                            23,
                                                        ),
                                                    ),
                                                    ty_expr: 0,
                                                },
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    20,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                25,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Type(
                                                TypeNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    24,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                19,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
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
                                            data: [
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        20,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    19,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        25,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                            ident_token: IdentToken {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    22,
                                                                ),
                                                            },
                                                            ty_expr_idx: 0,
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ConstantImplicitParameterType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        NodeDefn::ModuleItem(
            ModuleItemNodeDefn::Fugitive(
                FugitiveNodeDefn::Gn(
                    GnNodeDefn {
                        node_path: FugitiveNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                disambiguator: 0,
                            },
                        },
                        node_decl: GnNodeDecl {
                            node_path: FugitiveNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 8,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Fugitive(
                                                FugitiveNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    55,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                50,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Literal(
                                                TokenIdx(
                                                    69,
                                                ),
                                                Literal::Integer(
                                                    UnspecifiedRegular(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    74,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                50,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            Expr::ExplicitApplication {
                                                function_expr_idx: 4,
                                                argument_expr_idx: 5,
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `label`,
                                                token_idx: TokenIdx(
                                                    75,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Constant {
                                                        ident_token: IdentToken {
                                                            ident: `label`,
                                                            token_idx: TokenIdx(
                                                                53,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            Expr::ExplicitApplication {
                                                function_expr_idx: 6,
                                                argument_expr_idx: 7,
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            63,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `i32`,
                                                        token_idx: TokenIdx(
                                                            67,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `OneVsAllResult`,
                                                        token_idx: TokenIdx(
                                                            73,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                                            data: [
                                                PatternExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `skip`,
                                                        token_idx: TokenIdx(
                                                            65,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                None,
                                            ],
                                        },
                                        pattern_infos: [
                                            Parameter,
                                        ],
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                PatternSymbol::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `skip`,
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                None,
                                            ],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        51,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        56,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                            ident_token: IdentToken {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    53,
                                                                ),
                                                            },
                                                            ty_expr_idx: 0,
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        59,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitVariadicParameter {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `f`,
                                                            token_idx: TokenIdx(
                                                                61,
                                                            ),
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        66,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                        ident: `skip`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                ExplicitVariadicParameter {
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 2,
                                                },
                                                ArenaIdxRange(
                                                    3..4,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ConstantImplicitParameterType,
                                            expr_idx: 0,
                                        },
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 1,
                                        },
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 2,
                                        },
                                        ExprRoot {
                                            kind: ExplicitParameterDefaultValue {
                                                ty_expr_idx: 2,
                                            },
                                            expr_idx: 3,
                                        },
                                        ExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 8,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: Ok(
                                Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                49,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: ImplicitParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `Label`,
                                                        token_idx: TokenIdx(
                                                            50,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            ImplicitParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: ImplicitParameterDeclPatternVariant::Constant {
                                                    const_token: ConstToken {
                                                        token_idx: TokenIdx(
                                                            52,
                                                        ),
                                                    },
                                                    ident_token: IdentToken {
                                                        ident: `label`,
                                                        token_idx: TokenIdx(
                                                            53,
                                                        ),
                                                    },
                                                    colon_token: ColonToken(
                                                        TokenIdx(
                                                            54,
                                                        ),
                                                    ),
                                                    ty_expr: 0,
                                                },
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    51,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                56,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            explicit_parameter_decl_list: Ok(
                                SelfParameterAndExplicitParameters {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            57,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    explicit_parameters: [
                                        ExplicitParameterDecl::Variadic {
                                            dot_dot_dot_token: DotDotDotToken(
                                                TokenIdx(
                                                    58,
                                                ),
                                            ),
                                            variadic_variant: VariadicVariant::Vec {
                                                lbox_token: LeftBoxBracketToken(
                                                    TokenIdx(
                                                        59,
                                                    ),
                                                ),
                                                rbox_token: RightBoxBracketToken(
                                                    TokenIdx(
                                                        60,
                                                    ),
                                                ),
                                            },
                                            symbol_modifier_keyword_group: None,
                                            ident_token: IdentToken {
                                                ident: `f`,
                                                token_idx: TokenIdx(
                                                    61,
                                                ),
                                            },
                                            variable: 2,
                                            colon: ColonToken(
                                                TokenIdx(
                                                    62,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                        ExplicitParameterDecl::Keyed {
                                            pattern: 0,
                                            symbol_modifier_keyword_group: None,
                                            ident_token: IdentToken {
                                                ident: `skip`,
                                                token_idx: TokenIdx(
                                                    65,
                                                ),
                                            },
                                            variable: 3,
                                            colon: ColonToken(
                                                TokenIdx(
                                                    66,
                                                ),
                                            ),
                                            ty: 2,
                                            eq_token: EqToken(
                                                TokenIdx(
                                                    68,
                                                ),
                                            ),
                                            default: Right(
                                                3,
                                            ),
                                        },
                                    ],
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                64,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                70,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            71,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                Some(
                                    CurryToken(
                                        TokenIdx(
                                            72,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeExprBeforeColon {
                                        expr: 8,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolToken::Semicolon(
                                    EolSemicolonToken {
                                        token_idx: TokenIdx(
                                            76,
                                        ),
                                    },
                                ),
                            ),
                        },
                        body: None,
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntityNodePath::ModuleItem(
                                                    ModuleItemNodePath::Fugitive(
                                                        FugitiveNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::CurrentSymbol {
                                                        ident: `Label`,
                                                        token_idx: TokenIdx(
                                                            55,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        50,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::i32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            69,
                                                        ),
                                                        Literal::Integer(
                                                            UnspecifiedRegular(
                                                                5,
                                                            ),
                                                        ),
                                                    ),
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 2,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `Label`,
                                                        token_idx: TokenIdx(
                                                            74,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        50,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function_expr_idx: 4,
                                                        argument_expr_idx: 5,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `label`,
                                                        token_idx: TokenIdx(
                                                            75,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Constant {
                                                                ident_token: IdentToken {
                                                                    ident: `label`,
                                                                    token_idx: TokenIdx(
                                                                        53,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function_expr_idx: 6,
                                                        argument_expr_idx: 7,
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    63,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `i32`,
                                                                token_idx: TokenIdx(
                                                                    67,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `OneVsAllResult`,
                                                                token_idx: TokenIdx(
                                                                    73,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                                                    data: [
                                                        PatternExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `skip`,
                                                                token_idx: TokenIdx(
                                                                    65,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        None,
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `skip`,
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [
                                                        None,
                                                    ],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSymbol {
                                                            modifier: Const,
                                                            access_start: TokenIdx(
                                                                51,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `Label`,
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Const,
                                                            access_start: TokenIdx(
                                                                56,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                                    ident_token: IdentToken {
                                                                        ident: `label`,
                                                                        token_idx: TokenIdx(
                                                                            53,
                                                                        ),
                                                                    },
                                                                    ty_expr_idx: 0,
                                                                },
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                59,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitVariadicParameter {
                                                                symbol_modifier_keyword_group: None,
                                                                ident_token: IdentToken {
                                                                    ident: `f`,
                                                                    token_idx: TokenIdx(
                                                                        61,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                66,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                                ident: `skip`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        ImplicitTypeParameter,
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                    (
                                                        ImplicitTypeParameter,
                                                        ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitVariadicParameter {
                                                            ty: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            2..3,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 2,
                                                        },
                                                        ArenaIdxRange(
                                                            3..4,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ConstantImplicitParameterType,
                                                    expr_idx: 0,
                                                },
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 1,
                                                },
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 2,
                                                },
                                                ExprRoot {
                                                    kind: ExplicitParameterDefaultValue {
                                                        ty_expr_idx: 2,
                                                    },
                                                    expr_idx: 3,
                                                },
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 8,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_entity_path_expr_arena: Arena {
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
                                        data: [
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Type {
                                                        ident: `Label`,
                                                    },
                                                ),
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSymbol::Constant {
                                                        ident: `label`,
                                                    },
                                                ),
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                modifier: None,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `f`,
                                                },
                                            },
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                modifier: None,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `skip`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                    },
                ),
            ),
        ),
        NodeDefn::ImplBlock(
            ImplBlockNodeDecl::IllFormed(
                IllFormedImplBlockNodeDecl {
                    node_path: IllFormedImplBlockNodePath {
                        path: IllFormedImplBlockPath {
                            module_path: `malamute`,
                            disambiguator: 0,
                        },
                    },
                    ast_idx: 7,
                    expr_region: ExprRegion {
                        data: ExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                EntityNodePath::ImplBlock(
                                    ImplBlockNodePath::IllFormedImplBlock(
                                        IllFormedImplBlockNodePath {
                                            path: IllFormedImplBlockPath {
                                                module_path: `malamute`,
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            principal_entity_path_expr_arena: Arena {
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
                                allow_self_value: False,
                                pattern_ty_constraints: [],
                            },
                            roots: [],
                        },
                    },
                },
            ),
        ),
    ],
)