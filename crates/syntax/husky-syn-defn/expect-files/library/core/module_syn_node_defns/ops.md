Ok(
    [
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::Add`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::Add`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 30,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            10,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        11,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RightAngleBracketToken(
                                        TokenIdx(
                                            12,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Add`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    12,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                11,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::AddAssign`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::AddAssign`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 32,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            39,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        40,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RightAngleBracketToken(
                                        TokenIdx(
                                            41,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::AddAssign`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    41,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                40,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::BitAnd`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::BitAnd`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 34,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            68,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RightAngleBracketToken(
                                        TokenIdx(
                                            70,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitAnd`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    70,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                69,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::BitAndAssign`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::BitAndAssign`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 36,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            97,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        98,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RightAngleBracketToken(
                                        TokenIdx(
                                            99,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitAndAssign`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    99,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                98,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::BitOr`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::BitOr`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 38,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            122,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        123,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RightAngleBracketToken(
                                        TokenIdx(
                                            124,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitOr`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    124,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                123,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::BitOrAssign`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::BitOrAssign`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 40,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            151,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        152,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RightAngleBracketToken(
                                        TokenIdx(
                                            153,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitOrAssign`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    153,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                152,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::BitXor`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::BitXor`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 42,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            176,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        177,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RightAngleBracketToken(
                                        TokenIdx(
                                            178,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitXor`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    178,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                177,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::BitXorAssign`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::BitXorAssign`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 44,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            205,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        206,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RightAngleBracketToken(
                                        TokenIdx(
                                            207,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::BitXorAssign`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    207,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                206,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::Div`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::Div`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 46,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            230,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        231,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RightAngleBracketToken(
                                        TokenIdx(
                                            232,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Div`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    232,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                231,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::DivAssign`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::DivAssign`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 48,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            259,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        260,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RightAngleBracketToken(
                                        TokenIdx(
                                            261,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::DivAssign`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    261,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                260,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::Mul`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::Mul`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 50,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            284,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        285,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RightAngleBracketToken(
                                        TokenIdx(
                                            286,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Mul`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    286,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                285,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::MulAssign`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::MulAssign`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 52,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            313,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        314,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RightAngleBracketToken(
                                        TokenIdx(
                                            315,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::MulAssign`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    315,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                314,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::Neg`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::Neg`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 54,
                        template_parameter_decl_list: Ok(
                            None,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Neg`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
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
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::Not`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::Not`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 56,
                        template_parameter_decl_list: Ok(
                            None,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Not`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
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
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::Sub`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::Sub`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 58,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            386,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `Rhs`,
                                                    token_idx: TokenIdx(
                                                        387,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RightAngleBracketToken(
                                        TokenIdx(
                                            388,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Sub`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    388,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Rhs`,
                                                            token_idx: TokenIdx(
                                                                387,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Trait(
                TraitSynNodeDefn {
                    syn_node_path: TraitSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitPath(`core::ops::Unveil`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::ops::Unveil`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 59,
                        template_parameter_decl_list: Ok(
                            Some(
                                Generics {
                                    langle: LeftAngleBracketOrLessThanToken(
                                        TokenIdx(
                                            408,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterDecl {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterDeclPatternVariant::Type {
                                                ident_token: IdentToken {
                                                    ident: `T`,
                                                    token_idx: TokenIdx(
                                                        409,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RightAngleBracketToken(
                                        TokenIdx(
                                            410,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Trait(
                                            TraitSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`core::ops::Unveil`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: TokenIdx(
                                                    410,
                                                ),
                                                access_end: None,
                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `T`,
                                                            token_idx: TokenIdx(
                                                                409,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [],
                            },
                        },
                    },
                },
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Type(
                TypeSynNodeDefn::Enum(
                    EnumTypeSynNodeDefn {
                        syn_node_path: TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: EnumTypeSynNodeDecl {
                            syn_node_path: TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 60,
                            template_parameter_decl_list: Ok(
                                Some(
                                    Generics {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                437,
                                            ),
                                        ),
                                        template_parameters: [
                                            TemplateParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: TemplateParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `R`,
                                                        token_idx: TokenIdx(
                                                            438,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            TemplateParameterDecl {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: TemplateParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `C`,
                                                        token_idx: TokenIdx(
                                                            440,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    439,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                441,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternSynExprRegion {
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
                                    symbol_region: SynSymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        439,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `R`,
                                                                token_idx: TokenIdx(
                                                                    438,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        441,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentToken {
                                                                ident: `C`,
                                                                token_idx: TokenIdx(
                                                                    440,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
    ],
)