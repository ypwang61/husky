```rust
[
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath::MajorItem(
                MajorItemPath::Form(
                    FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                ),
            ),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath::MajorItem(
                    MajorItemPath::Form(
                        FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                    ),
                ),
            ),
            place_registry: PlaceRegistry {
                infos: [],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Form(
                                                    FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                                        Fn,
                                                    )`),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Form(
                                                    FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash Vec ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            FormPath(`mnist_classifier::digits::three::downarc`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::three::downarc`, `Ritchie(
                                                        Fn,
                                                    )`),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            FormPath(`mnist_classifier::digits::three::uparc`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::three::uparc`, `Ritchie(
                                                        Fn,
                                                    )`),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            FormPath(`mnist_classifier::digits::three::back`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::three::back`, `Ritchie(
                                                        Fn,
                                                    )`),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::NewList {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    items: [
                                        SemaCommaListItem {
                                            sem_expr_idx: SemExprIdx(
                                                2,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    7,
                                                ),
                                            ),
                                        },
                                        SemaCommaListItem {
                                            sem_expr_idx: SemExprIdx(
                                                3,
                                            ),
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                        },
                                        SemaCommaListItem {
                                            sem_expr_idx: SemExprIdx(
                                                4,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    element_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function_sem_expr_idx: SemExprIdx(
                                        0,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Leash Vec ConcaveComponent`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    1,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: Leashed {
                                                                    place_idx: None,
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::Comma(
                                                    RegionalTokenIdx(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        ),
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    5,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Block {
                                    stmts: SemStmtIdxRange(
                                        ArenaIdxRange(
                                            0..1,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    sem_expr_idx: SemExprIdx(
                                        6,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coercion(
                                            ExpectCoercionOutcome {
                                                coercion: Trivial(
                                                    TrivialFlyCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    7,
                    (
                        SemExprIdx(
                            7,
                        ),
                        SynExprRootKind::BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [],
            },
            sem_expr_terms: [],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    sol_terms: SolTerms {
                        entries: [],
                    },
                    hol_terms: HolTerms {
                        entries: [],
                        first_unresolved_term_idx: 0,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 0,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash Vec ConcaveComponent,  Vec fn(( Leash ConcaveComponent) -> Option f32) -> FermiMatchResult`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`FermiMatchResult`),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Pure,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`Leash Vec ConcaveComponent`),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Pure,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Eth(
                                                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Leash Vec ConcaveComponent`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Leashed {
                                                                place_idx: None,
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`FermiMatchResult`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`FermiMatchResult`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: Some(
                EthTerm(`FermiMatchResult`),
            ),
            self_ty: None,
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath::MajorItem(
                MajorItemPath::Form(
                    FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                ),
            ),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath::MajorItem(
                    MajorItemPath::Form(
                        FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                    ),
                ),
            ),
            place_registry: PlaceRegistry {
                infos: [
                    PlaceInfo::Variable {
                        current_variable_idx: 0,
                        ident: `downarc`,
                    },
                    PlaceInfo::Variable {
                        current_variable_idx: 1,
                        ident: `uparc`,
                    },
                    PlaceInfo::Variable {
                        current_variable_idx: 2,
                        ident: `back`,
                    },
                    PlaceInfo::Variable {
                        current_variable_idx: 3,
                        ident: `de`,
                    },
                    PlaceInfo::Variable {
                        current_variable_idx: 4,
                        ident: `downarc_enpoint`,
                    },
                    PlaceInfo::Variable {
                        current_variable_idx: 5,
                        ident: `uparc_startpoint`,
                    },
                    PlaceInfo::Variable {
                        current_variable_idx: 6,
                        ident: `distance`,
                    },
                ],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Form(
                                                    FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash Vec ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemExprIdx(
                                        0,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            4,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::ilen`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Vec ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(
                                                                `core::vec::Vec(0)::ilen`,
                                                                TypeItemKind::MethodRitchie(
                                                                    RitchieItemKind::Fn,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    task_ty: Some(
                                                        EthTerm(`MnistTask`),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
                                                        },
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EthSymbolicVariable(`E`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    place: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`ConcaveComponent`),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        8,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            2,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        1,
                                    ),
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ropd: SemExprIdx(
                                        2,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Form(
                                                    FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash Vec ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemExprIdx(
                                        4,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ilen`,
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::ilen`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Vec ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`i32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(
                                                                `core::vec::Vec(0)::ilen`,
                                                                TypeItemKind::MethodRitchie(
                                                                    RitchieItemKind::Fn,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    task_ty: Some(
                                                        EthTerm(`MnistTask`),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
                                                        },
                                                    },
                                                    symbol_map: [
                                                        (
                                                            EthSymbolicVariable(`E`),
                                                            FlyTermSymbolResolution::Explicit(
                                                                FlyTerm {
                                                                    place: None,
                                                                    base: FlyTermBase::Eth(
                                                                        EthTerm(`ConcaveComponent`),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        16,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            4,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`i32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        5,
                                    ),
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    ropd: SemExprIdx(
                                        6,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Field {
                                    self_argument: SemExprIdx(
                                        8,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        24,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            0,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Index {
                                    owner: SemExprIdx(
                                        9,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    index_sem_list_items: [
                                        SemaCommaListItem {
                                            sem_expr_idx: SemExprIdx(
                                                10,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                    index_dynamic_dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    13,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Field {
                                    self_argument: SemExprIdx(
                                        12,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        30,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            31,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    14,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        33,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    15,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Index {
                                    owner: SemExprIdx(
                                        13,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    index_sem_list_items: [
                                        SemaCommaListItem {
                                            sem_expr_idx: SemExprIdx(
                                                14,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    index_dynamic_dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    17,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 7,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    18,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Field {
                                    self_argument: SemExprIdx(
                                        16,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            40,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Vec Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vec Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    19,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        42,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            2,
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    20,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Index {
                                    owner: SemExprIdx(
                                        17,
                                    ),
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                    index_sem_list_items: [
                                        SemaCommaListItem {
                                            sem_expr_idx: SemExprIdx(
                                                18,
                                            ),
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        43,
                                    ),
                                    index_dynamic_dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: FlyIndexSignature::Int {
                                            element_ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Option Leash ConcaveComponent`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    22,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `downarc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    23,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Be {
                                    src: SemExprIdx(
                                        20,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    target: BePatternSyndicate {
                                        pattern_expr_root: BeSynPatternRoot {
                                            syn_pattern_idx: 4,
                                        },
                                        variables: ArenaIdxRange(
                                            3..3,
                                        ),
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    24,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `downarc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    25,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Unwrap {
                                    opd_sem_expr_idx: SemExprIdx(
                                        22,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        53,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    26,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Field {
                                    self_argument: SemExprIdx(
                                        23,
                                    ),
                                    self_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        54,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            55,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(
                                                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`,
                                                    TypeItemKind::MemoizedField,
                                                ),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(
                                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`,
                                                            TypeItemKind::MemoizedField,
                                                        ),
                                                    ),
                                                ),
                                                task_ty: Some(
                                                    EthTerm(`MnistTask`),
                                                ),
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    27,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        57,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 77,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    28,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        24,
                                    ),
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                    ropd: SemExprIdx(
                                        25,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    29,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `uparc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        59,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(1),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    30,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Be {
                                    src: SemExprIdx(
                                        27,
                                    ),
                                    be_regional_token_idx: RegionalTokenIdx(
                                        60,
                                    ),
                                    target: BePatternSyndicate {
                                        pattern_expr_root: BeSynPatternRoot {
                                            syn_pattern_idx: 6,
                                        },
                                        variables: ArenaIdxRange(
                                            3..3,
                                        ),
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    31,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `downarc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        68,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    32,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Unwrap {
                                    opd_sem_expr_idx: SemExprIdx(
                                        29,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        69,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    33,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemExprIdx(
                                        30,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        70,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end_tangent`,
                                        regional_token_idx: RegionalTokenIdx(
                                            71,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Vector2d`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(
                                                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`,
                                                                TypeItemKind::MethodRitchie(
                                                                    RitchieItemKind::Fn,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    task_ty: Some(
                                                        EthTerm(`MnistTask`),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        72,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        73,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    34,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        77,
                                    ),
                                    LiteralTokenData::Bool(
                                        True,
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    35,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemExprIdx(
                                        31,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        74,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle`,
                                        regional_token_idx: RegionalTokenIdx(
                                            75,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::geom2d::Vector2d(0)::angle`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Vector2d`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FlyRitchieParameter::Simple(
                                                        FlyRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: FlyTerm {
                                                                place: None,
                                                                base: FlyTermBase::Eth(
                                                                    EthTerm(`bool`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(
                                                                `mnist_classifier::geom2d::Vector2d(0)::angle`,
                                                                TypeItemKind::MethodRitchie(
                                                                    RitchieItemKind::Fn,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    task_ty: Some(
                                                        EthTerm(`MnistTask`),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Transient,
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        76,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`bool`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    32,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: Compterm,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        78,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    36,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `de`,
                                    regional_token_idx: RegionalTokenIdx(
                                        80,
                                    ),
                                    current_variable_idx: 3,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 5,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(3),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    37,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(3),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        82,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 78,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    38,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        34,
                                    ),
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        81,
                                    ),
                                    ropd: SemExprIdx(
                                        35,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    39,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `de`,
                                    regional_token_idx: RegionalTokenIdx(
                                        84,
                                    ),
                                    current_variable_idx: 3,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 5,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(3),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    40,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(3),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        87,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            3,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    41,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        86,
                                    ),
                                    opd: SemExprIdx(
                                        38,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            3,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    42,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        37,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        85,
                                    ),
                                    ropd: SemExprIdx(
                                        39,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    43,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        36,
                                    ),
                                    opr: ShortCircuitLogic(
                                        Or,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        83,
                                    ),
                                    ropd: SemExprIdx(
                                        40,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    44,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `downarc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        91,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    45,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Unwrap {
                                    opd_sem_expr_idx: SemExprIdx(
                                        42,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        92,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    46,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemExprIdx(
                                        43,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        93,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `end`,
                                        regional_token_idx: RegionalTokenIdx(
                                            94,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Point2d`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(
                                                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`,
                                                                TypeItemKind::MethodRitchie(
                                                                    RitchieItemKind::Fn,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    task_ty: Some(
                                                        EthTerm(`MnistTask`),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        95,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        96,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    47,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `uparc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        100,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(1),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    48,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Unwrap {
                                    opd_sem_expr_idx: SemExprIdx(
                                        45,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        101,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    49,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemExprIdx(
                                        46,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        102,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `start`,
                                        regional_token_idx: RegionalTokenIdx(
                                            103,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Point2d`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(
                                                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`,
                                                                TypeItemKind::MethodRitchie(
                                                                    RitchieItemKind::Fn,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    task_ty: Some(
                                                        EthTerm(`MnistTask`),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        104,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        105,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    50,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `downarc_enpoint`,
                                    regional_token_idx: RegionalTokenIdx(
                                        109,
                                    ),
                                    current_variable_idx: 4,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 6,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(4),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    51,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(4),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `uparc_startpoint`,
                                    regional_token_idx: RegionalTokenIdx(
                                        113,
                                    ),
                                    current_variable_idx: 5,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 7,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(5),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Point2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    52,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(5),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Point2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemExprIdx(
                                        48,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        110,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `dist`,
                                        regional_token_idx: RegionalTokenIdx(
                                            111,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(4),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(4),
                                                ),
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::geom2d::Point2d(0)::dist`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`Point2d`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [
                                                    FlyRitchieParameter::Simple(
                                                        FlyRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: FlyTerm {
                                                                place: None,
                                                                base: FlyTermBase::Eth(
                                                                    EthTerm(`Point2d`),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(
                                                                `mnist_classifier::geom2d::Point2d(0)::dist`,
                                                                TypeItemKind::MethodRitchie(
                                                                    RitchieItemKind::Fn,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    task_ty: Some(
                                                        EthTerm(`MnistTask`),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: ImmutableOnStack {
                                                            place: Idx(
                                                                PlaceIdx(4),
                                                            ),
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        112,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Pure,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Point2d`),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    49,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: ImmutableOnStack {
                                                                    place: Idx(
                                                                        PlaceIdx(5),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        114,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    53,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `distance`,
                                    regional_token_idx: RegionalTokenIdx(
                                        116,
                                    ),
                                    current_variable_idx: 6,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 8,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(6),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    54,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(6),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        118,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 80,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    55,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        51,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        117,
                                    ),
                                    ropd: SemExprIdx(
                                        52,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    56,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 10,
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::MajorItem(
                                                MajorItemPath::Form(
                                                    FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`FermiMatchResult`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    57,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Field {
                                    self_argument: SemExprIdx(
                                        54,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        121,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `norm`,
                                        regional_token_idx: RegionalTokenIdx(
                                            122,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(
                                                    `mnist_classifier::fermi::FermiMatchResult(0)::norm`,
                                                    TypeItemKind::MemoizedField,
                                                ),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(
                                                            `mnist_classifier::fermi::FermiMatchResult(0)::norm`,
                                                            TypeItemKind::MemoizedField,
                                                        ),
                                                    ),
                                                ),
                                                task_ty: Some(
                                                    EthTerm(`MnistTask`),
                                                ),
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    58,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        124,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 81,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    59,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        55,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        123,
                                    ),
                                    ropd: SemExprIdx(
                                        56,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    60,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `downarc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        126,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Option Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    61,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Unwrap {
                                    opd_sem_expr_idx: SemExprIdx(
                                        58,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        127,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    62,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Field {
                                    self_argument: SemExprIdx(
                                        59,
                                    ),
                                    self_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        128,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `angle_change`,
                                        regional_token_idx: RegionalTokenIdx(
                                            129,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(
                                                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`,
                                                    TypeItemKind::MemoizedField,
                                                ),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(
                                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`,
                                                            TypeItemKind::MemoizedField,
                                                        ),
                                                    ),
                                                ),
                                                task_ty: Some(
                                                    EthTerm(`MnistTask`),
                                                ),
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    63,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        132,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 82,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    64,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                4,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        131,
                                    ),
                                    opd: SemExprIdx(
                                        61,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    65,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                4,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        60,
                                    ),
                                    opr: Comparison(
                                        Less,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        130,
                                    ),
                                    ropd: SemExprIdx(
                                        62,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    66,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 12,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 265,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::TypeVariant(
                                                Room32,
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 265,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`Label`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            place: None,
                                                            base: FlyTermBase::Hol(
                                                                HolTerm(
                                                                    5,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                (
                                                    EthSymbolicVariable(`label`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            place: None,
                                                            base: FlyTermBase::Hol(
                                                                HolTerm(
                                                                    6,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    67,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Block {
                                    stmts: SemStmtIdxRange(
                                        ArenaIdxRange(
                                            0..17,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    70,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemExprIdx(
                                            3,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemExprIdx(
                                            7,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            17,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 0,
                                        },
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            19,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemExprIdx(
                                        11,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            26,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 1,
                                        },
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            28,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemExprIdx(
                                        15,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            35,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 2,
                                        },
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            37,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemExprIdx(
                                        19,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            44,
                                        ),
                                    },
                                    condition: Be {
                                        src: SemExprIdx(
                                            20,
                                        ),
                                        be_regional_token_idx: RegionalTokenIdx(
                                            46,
                                        ),
                                        target: BePatternSyndicate {
                                            pattern_expr_root: BeSynPatternRoot {
                                                syn_pattern_idx: 4,
                                            },
                                            variables: ArenaIdxRange(
                                                3..3,
                                            ),
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            51,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemExprIdx(
                                            26,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            58,
                                        ),
                                    },
                                    condition: Be {
                                        src: SemExprIdx(
                                            27,
                                        ),
                                        be_regional_token_idx: RegionalTokenIdx(
                                            60,
                                        ),
                                        target: BePatternSyndicate {
                                            pattern_expr_root: BeSynPatternRoot {
                                                syn_pattern_idx: 6,
                                            },
                                            variables: ArenaIdxRange(
                                                3..3,
                                            ),
                                        },
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            65,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 7,
                                        },
                                        variables: ArenaIdxRange(
                                            3..4,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            67,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemExprIdx(
                                        33,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            79,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemExprIdx(
                                            41,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            88,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 8,
                                        },
                                        variables: ArenaIdxRange(
                                            4..5,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            90,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemExprIdx(
                                        44,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            97,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 9,
                                        },
                                        variables: ArenaIdxRange(
                                            5..6,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            99,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemExprIdx(
                                        47,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            106,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 10,
                                        },
                                        variables: ArenaIdxRange(
                                            6..7,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            108,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemExprIdx(
                                        50,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            115,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemExprIdx(
                                            53,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            119,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemExprIdx(
                                            57,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            125,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemExprIdx(
                                            63,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    sem_expr_idx: SemExprIdx(
                                        64,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coercion(
                                            ExpectCoercionOutcome {
                                                coercion: Trivial(
                                                    TrivialFlyCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    65,
                    (
                        SemExprIdx(
                            65,
                        ),
                        SynExprRootKind::BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: Some(
                                Leashed {
                                    place_idx: None,
                                },
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 58,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: Some(
                                Leashed {
                                    place_idx: None,
                                },
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 58,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: Some(
                                Leashed {
                                    place_idx: None,
                                },
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 58,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 58,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(1),
                                    ),
                                },
                            ),
                            base: Eth(
                                Application(
                                    EthApplication(
                                        Id {
                                            value: 58,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: None,
                            base: Eth(
                                ItemPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 138,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: None,
                            base: Eth(
                                ItemPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 30,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: None,
                            base: Eth(
                                ItemPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 30,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: None,
                            base: Eth(
                                ItemPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 138,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: Eth(
                                        Application(
                                            EthApplication(
                                                Id {
                                                    value: 58,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: Eth(
                                        Application(
                                            EthApplication(
                                                Id {
                                                    value: 58,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: Eth(
                                        Application(
                                            EthApplication(
                                                Id {
                                                    value: 58,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    None,
                    None,
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: None,
                                    base: Eth(
                                        ItemPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 138,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: None,
                                    base: Eth(
                                        ItemPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 30,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: None,
                                    base: Eth(
                                        ItemPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 30,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: None,
                                    base: Eth(
                                        ItemPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 138,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            sem_expr_terms: [
                (
                    SemExprIdx(
                        2,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`2`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        6,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`4`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        10,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        14,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`1`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        18,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`2`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        25,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`3.0f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        32,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`true`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        35,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0.0f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        38,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`100.0f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        52,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`20.0f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        56,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`2.5f32`),
                            ),
                        },
                    ),
                ),
                (
                    SemExprIdx(
                        61,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`100.0f32`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Option Leash ConcaveComponent`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(1),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Option Leash ConcaveComponent`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(2),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Option Leash ConcaveComponent`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(3),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`f32`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(4),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Point2d`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(5),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Point2d`),
                            ),
                        },
                    ),
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(6),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`f32`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    sol_terms: SolTerms {
                        entries: [],
                    },
                    hol_terms: HolTerms {
                        entries: [
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        10,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`usize`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        14,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`usize`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        18,
                                    ),
                                    hole_kind: UnspecifiedIntegerType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`usize`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`usize`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        36,
                                    ),
                                    hole_kind: UnspecifiedFloatType,
                                    fill: Some(
                                        FlyTerm {
                                            place: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FlyTerm {
                                                place: Some(
                                                    ImmutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                    },
                                                ),
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`f32`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        60,
                                    ),
                                    hole_kind: UnspecifiedFloatType,
                                    fill: Some(
                                        FlyTerm {
                                            place: Some(
                                                Leashed {
                                                    place_idx: None,
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleInto {
                                            target: FlyTerm {
                                                place: Some(
                                                    Leashed {
                                                        place_idx: None,
                                                    },
                                                ),
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`f32`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        64,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`MnistLabel`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`MnistLabel`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`MnistLabel`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        64,
                                    ),
                                    hole_kind: AnyOriginal,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Three`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::Subtype {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`Three`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Three`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                                    refined_path: Right(
                                        OtherTypePath(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    5,
                                                ),
                                            ),
                                        },
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`OneVsAll MnistLabel Three`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 7,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 0,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash Vec ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`i32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`i32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                0,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 13,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 14,
                                    src: ExpectationSource {
                                        syn_expr_idx: 13,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 15,
                                    src: ExpectationSource {
                                        syn_expr_idx: 14,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 16,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 17,
                                    src: ExpectationSource {
                                        syn_expr_idx: 15,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 18,
                                    src: ExpectationSource {
                                        syn_expr_idx: 16,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 19,
                                    src: ExpectationSource {
                                        syn_expr_idx: 17,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vec Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 20,
                                    src: ExpectationSource {
                                        syn_expr_idx: 18,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`usize`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 21,
                                    src: ExpectationSource {
                                        syn_expr_idx: 19,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 22,
                                    src: ExpectationSource {
                                        syn_expr_idx: 19,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 23,
                                    src: ExpectationSource {
                                        syn_expr_idx: 20,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 24,
                                    src: ExpectationSource {
                                        syn_expr_idx: 21,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 25,
                                    src: ExpectationSource {
                                        syn_expr_idx: 22,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 26,
                                    src: ExpectationSource {
                                        syn_expr_idx: 23,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 27,
                                    src: ExpectationSource {
                                        syn_expr_idx: 24,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                Leashed {
                                                    place_idx: None,
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 28,
                                    src: ExpectationSource {
                                        syn_expr_idx: 25,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 29,
                                    src: ExpectationSource {
                                        syn_expr_idx: 26,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 30,
                                    src: ExpectationSource {
                                        syn_expr_idx: 27,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 31,
                                    src: ExpectationSource {
                                        syn_expr_idx: 28,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 32,
                                    src: ExpectationSource {
                                        syn_expr_idx: 29,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 33,
                                    src: ExpectationSource {
                                        syn_expr_idx: 30,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 34,
                                    src: ExpectationSource {
                                        syn_expr_idx: 31,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`bool`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 35,
                                    src: ExpectationSource {
                                        syn_expr_idx: 32,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 36,
                                    src: ExpectationSource {
                                        syn_expr_idx: 33,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 37,
                                    src: ExpectationSource {
                                        syn_expr_idx: 34,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(3),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 38,
                                    src: ExpectationSource {
                                        syn_expr_idx: 35,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`bool`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 39,
                                    src: ExpectationSource {
                                        syn_expr_idx: 39,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 40,
                                    src: ExpectationSource {
                                        syn_expr_idx: 37,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(3),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 41,
                                    src: ExpectationSource {
                                        syn_expr_idx: 36,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 42,
                                    src: ExpectationSource {
                                        syn_expr_idx: 38,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                3,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`bool`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 43,
                                    src: ExpectationSource {
                                        syn_expr_idx: 40,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 44,
                                    src: ExpectationSource {
                                        syn_expr_idx: 41,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 45,
                                    src: ExpectationSource {
                                        syn_expr_idx: 42,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 46,
                                    src: ExpectationSource {
                                        syn_expr_idx: 43,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 47,
                                    src: ExpectationSource {
                                        syn_expr_idx: 44,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Point2d`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 48,
                                    src: ExpectationSource {
                                        syn_expr_idx: 45,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 49,
                                    src: ExpectationSource {
                                        syn_expr_idx: 46,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 50,
                                    src: ExpectationSource {
                                        syn_expr_idx: 47,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Point2d`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 51,
                                    src: ExpectationSource {
                                        syn_expr_idx: 48,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(4),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Point2d`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Point2d`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 52,
                                    src: ExpectationSource {
                                        syn_expr_idx: 49,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(5),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Point2d`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: ImmutableOnStack {
                                                                place: Idx(
                                                                    PlaceIdx(5),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 53,
                                    src: ExpectationSource {
                                        syn_expr_idx: 50,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 54,
                                    src: ExpectationSource {
                                        syn_expr_idx: 51,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(6),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(6),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 55,
                                    src: ExpectationSource {
                                        syn_expr_idx: 52,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 56,
                                    src: ExpectationSource {
                                        syn_expr_idx: 53,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 57,
                                    src: ExpectationSource {
                                        syn_expr_idx: 54,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`FermiMatchResult`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 58,
                                    src: ExpectationSource {
                                        syn_expr_idx: 55,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                Leashed {
                                                    place_idx: None,
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 59,
                                    src: ExpectationSource {
                                        syn_expr_idx: 56,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 60,
                                    src: ExpectationSource {
                                        syn_expr_idx: 57,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 61,
                                    src: ExpectationSource {
                                        syn_expr_idx: 58,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Option Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 62,
                                    src: ExpectationSource {
                                        syn_expr_idx: 59,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 63,
                                    src: ExpectationSource {
                                        syn_expr_idx: 61,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 64,
                                    src: ExpectationSource {
                                        syn_expr_idx: 60,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                4,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                Leashed {
                                                    place_idx: None,
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 65,
                                    src: ExpectationSource {
                                        syn_expr_idx: 62,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                4,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 66,
                                    src: ExpectationSource {
                                        syn_expr_idx: 63,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`OneVsAll MnistLabel Three`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 67,
                                    src: ExpectationSource {
                                        syn_expr_idx: 64,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`MnistLabel`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 68,
                                    src: ExpectationSource {
                                        syn_expr_idx: 64,
                                        kind: Expectation(
                                            67,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                5,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsExactly(
                                    ExpectSubtypeOrEqual {
                                        expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Three`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 69,
                                    src: ExpectationSource {
                                        syn_expr_idx: 64,
                                        kind: Expectation(
                                            67,
                                        ),
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                6,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Subtype(
                                                ExpectSubtypeOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`OneVsAll MnistLabel Three`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 70,
                                    src: ExpectationSource {
                                        syn_expr_idx: 65,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                7,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: Some(
                EthTerm(`OneVsAll MnistLabel Three`),
            ),
            self_ty: None,
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath::MajorItem(
                MajorItemPath::Form(
                    FormPath(`mnist_classifier::digits::three::uparc`, `Ritchie(
                        Fn,
                    )`),
                ),
            ),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath::MajorItem(
                    MajorItemPath::Form(
                        FormPath(`mnist_classifier::digits::three::uparc`, `Ritchie(
                            Fn,
                        )`),
                    ),
                ),
            ),
            place_registry: PlaceRegistry {
                infos: [
                    PlaceInfo::Variable {
                        current_variable_idx: 0,
                        ident: `dp`,
                    },
                ],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_syn_symbol_idx: 0,
                                    inherited_syn_symbol_kind: InheritedVariableKind::Parenate {
                                        ident: `cc`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemExprIdx(
                                        0,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Vector2d`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(
                                                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                                                                TypeItemKind::MethodRitchie(
                                                                    RitchieItemKind::Fn,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    task_ty: Some(
                                                        EthTerm(`MnistTask`),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `dp`,
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Field {
                                    self_argument: SemExprIdx(
                                        2,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 83,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        3,
                                    ),
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    ropd: SemExprIdx(
                                        4,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 217,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::TypeVariant(
                                                Room32,
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 217,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`T`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            place: None,
                                                            base: FlyTermBase::Hol(
                                                                HolTerm(
                                                                    0,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    inherited_syn_symbol_idx: 0,
                                    inherited_syn_symbol_kind: InheritedVariableKind::Parenate {
                                        ident: `cc`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Field {
                                    self_argument: SemExprIdx(
                                        7,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`BoundingBox`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(
                                                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`,
                                                    TypeItemKind::MemoizedField,
                                                ),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(
                                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`,
                                                            TypeItemKind::MemoizedField,
                                                        ),
                                                    ),
                                                ),
                                                task_ty: Some(
                                                    EthTerm(`MnistTask`),
                                                ),
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`BoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`BoundingBox`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemExprIdx(
                                        8,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymin`,
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::geom2d::BoundingBox(0)::ymin`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`BoundingBox`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(
                                                                `mnist_classifier::geom2d::BoundingBox(0)::ymin`,
                                                                TypeItemKind::MethodRitchie(
                                                                    RitchieItemKind::Fn,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    task_ty: Some(
                                                        EthTerm(`MnistTask`),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    opd: SemExprIdx(
                                        9,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function_sem_expr_idx: SemExprIdx(
                                        6,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    10,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Block {
                                    stmts: SemStmtIdxRange(
                                        ArenaIdxRange(
                                            0..3,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 0,
                                        },
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemExprIdx(
                                        1,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemExprIdx(
                                            5,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    sem_expr_idx: SemExprIdx(
                                        11,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coercion(
                                            ExpectCoercionOutcome {
                                                coercion: Trivial(
                                                    TrivialFlyCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    12,
                    (
                        SemExprIdx(
                            12,
                        ),
                        SynExprRootKind::BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: None,
                            base: Eth(
                                ItemPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 32,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: None,
                                    base: Eth(
                                        ItemPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 32,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            sem_expr_terms: [
                (
                    SemExprIdx(
                        4,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0.0f32`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                StackPure {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Leash ConcaveComponent`),
                            ),
                        },
                    ),
                ],
                current_variable_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Vector2d`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    sol_terms: SolTerms {
                        entries: [],
                    },
                    hol_terms: HolTerms {
                        entries: [
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        6,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`f32`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`core::option::Option`, `Enum`),
                                    refined_path: Left(
                                        PreludeTypePath::Option,
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Option f32`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Ritchie {
                                    ritchie_kind: RitchieKind::Type(
                                        RitchieTypeKind::Item(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    params: [
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`fn((move  f32) -> Option f32`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 1,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 0,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Hol(
                                                            HolTerm(
                                                                1,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Move,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Hol(
                                                                            HolTerm(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`BoundingBox`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Option f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Option f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: Some(
                EthTerm(`Option f32`),
            ),
            self_ty: None,
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath::MajorItem(
                MajorItemPath::Form(
                    FormPath(`mnist_classifier::digits::three::downarc`, `Ritchie(
                        Fn,
                    )`),
                ),
            ),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath::MajorItem(
                    MajorItemPath::Form(
                        FormPath(`mnist_classifier::digits::three::downarc`, `Ritchie(
                            Fn,
                        )`),
                    ),
                ),
            ),
            place_registry: PlaceRegistry {
                infos: [
                    PlaceInfo::Variable {
                        current_variable_idx: 0,
                        ident: `dp`,
                    },
                ],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_syn_symbol_idx: 0,
                                    inherited_syn_symbol_kind: InheritedVariableKind::Parenate {
                                        ident: `cc`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemExprIdx(
                                        0,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Vector2d`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(
                                                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                                                                TypeItemKind::MethodRitchie(
                                                                    RitchieItemKind::Fn,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    task_ty: Some(
                                                        EthTerm(`MnistTask`),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `dp`,
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Field {
                                    self_argument: SemExprIdx(
                                        2,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 84,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        3,
                                    ),
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    ropd: SemExprIdx(
                                        4,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 217,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::TypeVariant(
                                                Room32,
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 217,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`T`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            place: None,
                                                            base: FlyTermBase::Hol(
                                                                HolTerm(
                                                                    0,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    inherited_syn_symbol_idx: 0,
                                    inherited_syn_symbol_kind: InheritedVariableKind::Parenate {
                                        ident: `cc`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Field {
                                    self_argument: SemExprIdx(
                                        7,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`BoundingBox`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(
                                                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`,
                                                    TypeItemKind::MemoizedField,
                                                ),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(
                                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`,
                                                            TypeItemKind::MemoizedField,
                                                        ),
                                                    ),
                                                ),
                                                task_ty: Some(
                                                    EthTerm(`MnistTask`),
                                                ),
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`BoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`BoundingBox`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemExprIdx(
                                        8,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymin`,
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::geom2d::BoundingBox(0)::ymin`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`BoundingBox`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(
                                                                `mnist_classifier::geom2d::BoundingBox(0)::ymin`,
                                                                TypeItemKind::MethodRitchie(
                                                                    RitchieItemKind::Fn,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    task_ty: Some(
                                                        EthTerm(`MnistTask`),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    opd: SemExprIdx(
                                        9,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function_sem_expr_idx: SemExprIdx(
                                        6,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    10,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Block {
                                    stmts: SemStmtIdxRange(
                                        ArenaIdxRange(
                                            0..3,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 0,
                                        },
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemExprIdx(
                                        1,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemExprIdx(
                                            5,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    sem_expr_idx: SemExprIdx(
                                        11,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coercion(
                                            ExpectCoercionOutcome {
                                                coercion: Trivial(
                                                    TrivialFlyCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    12,
                    (
                        SemExprIdx(
                            12,
                        ),
                        SynExprRootKind::BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: None,
                            base: Eth(
                                ItemPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 32,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: None,
                                    base: Eth(
                                        ItemPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 32,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            sem_expr_terms: [
                (
                    SemExprIdx(
                        4,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0.0f32`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                StackPure {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Leash ConcaveComponent`),
                            ),
                        },
                    ),
                ],
                current_variable_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Vector2d`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    sol_terms: SolTerms {
                        entries: [],
                    },
                    hol_terms: HolTerms {
                        entries: [
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        6,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`f32`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`core::option::Option`, `Enum`),
                                    refined_path: Left(
                                        PreludeTypePath::Option,
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Option f32`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Ritchie {
                                    ritchie_kind: RitchieKind::Type(
                                        RitchieTypeKind::Item(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    params: [
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`fn((move  f32) -> Option f32`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 1,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 0,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Hol(
                                                            HolTerm(
                                                                1,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Move,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Hol(
                                                                            HolTerm(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`BoundingBox`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Option f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Option f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: Some(
                EthTerm(`Option f32`),
            ),
            self_ty: None,
        },
    },
    SemExprRegion {
        path: RegionPath::ItemDefn(
            ItemPath::MajorItem(
                MajorItemPath::Form(
                    FormPath(`mnist_classifier::digits::three::back`, `Ritchie(
                        Fn,
                    )`),
                ),
            ),
        ),
        data: SemExprRegionData {
            path: RegionPath::ItemDefn(
                ItemPath::MajorItem(
                    MajorItemPath::Form(
                        FormPath(`mnist_classifier::digits::three::back`, `Ritchie(
                            Fn,
                        )`),
                    ),
                ),
            ),
            place_registry: PlaceRegistry {
                infos: [
                    PlaceInfo::Variable {
                        current_variable_idx: 0,
                        ident: `dp`,
                    },
                ],
            },
            sem_expr_arena: SemExprArena(
                Arena {
                    data: [
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_syn_symbol_idx: 0,
                                    inherited_syn_symbol_kind: InheritedVariableKind::Parenate {
                                        ident: `cc`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    0,
                                    FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemExprIdx(
                                        0,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `displacement`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`ConcaveComponent`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`Vector2d`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(
                                                                `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                                                                TypeItemKind::MethodRitchie(
                                                                    RitchieItemKind::Fn,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    task_ty: Some(
                                                        EthTerm(`MnistTask`),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    1,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::CurrentSynSymbol {
                                    ident: `dp`,
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Vector2d`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    2,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Field {
                                    self_argument: SemExprIdx(
                                        2,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `y`,
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [],
                                            final_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        signature: FlyFieldSignature::PropsStruct {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        ImmutableOnStack {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    3,
                                    FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Literal(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                    LiteralTokenData::Float(
                                        Unspecified(
                                            UnspecifiedFloatLiteral(
                                                Id {
                                                    value: 85,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Compterm,
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    4,
                                    FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Binary {
                                    lopd: SemExprIdx(
                                        3,
                                    ),
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Transient,
                                            indirections: [],
                                            final_place: Transient,
                                        },
                                        signature: SemaBinaryOprFlySignature::Builtin,
                                    },
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    ropd: SemExprIdx(
                                        4,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`bool`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    5,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath(
                                            ItemPathId(
                                                Id {
                                                    value: 217,
                                                },
                                            ),
                                        ),
                                    ),
                                    ty_path_disambiguation: InstanceConstructor,
                                    instantiation: Some(
                                        FlyInstantiation {
                                            path: ItemPath::TypeVariant(
                                                Room32,
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 217,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            task_ty: Some(
                                                EthTerm(`MnistTask`),
                                            ),
                                            env: TypeOntologyConstructor,
                                            symbol_map: [
                                                (
                                                    EthSymbolicVariable(`T`),
                                                    FlyTermSymbolResolution::Explicit(
                                                        FlyTerm {
                                                            place: None,
                                                            base: FlyTermBase::Hol(
                                                                HolTerm(
                                                                    0,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            separator: None,
                                        },
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            2,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    6,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::InheritedSynSymbol {
                                    ident: `cc`,
                                    regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    inherited_syn_symbol_idx: 0,
                                    inherited_syn_symbol_kind: InheritedVariableKind::Parenate {
                                        ident: `cc`,
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        StackPure {
                                            place: Idx(
                                                PlaceIdx(0),
                                            ),
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`Leash ConcaveComponent`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    7,
                                    FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Field {
                                    self_argument: SemExprIdx(
                                        7,
                                    ),
                                    self_ty: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `bounding_box`,
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                    dispatch: FlyFieldDyanmicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            indirections: [
                                                Leash,
                                            ],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        signature: FlyFieldSignature::Memoized {
                                            ty: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`BoundingBox`),
                                                ),
                                            },
                                            path: AssocItemPath::TypeItem(
                                                TypeItemPath(
                                                    `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`,
                                                    TypeItemKind::MemoizedField,
                                                ),
                                            ),
                                            instantiation: FlyInstantiation {
                                                path: ItemPath::AssocItem(
                                                    AssocItemPath::TypeItem(
                                                        TypeItemPath(
                                                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`,
                                                            TypeItemKind::MemoizedField,
                                                        ),
                                                    ),
                                                ),
                                                task_ty: Some(
                                                    EthTerm(`MnistTask`),
                                                ),
                                                env: MemoizedField,
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                        },
                                    },
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: Some(
                                        Leashed {
                                            place_idx: None,
                                        },
                                    ),
                                    base: FlyTermBase::Eth(
                                        EthTerm(`BoundingBox`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    8,
                                    FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`BoundingBox`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::MethodFnCall {
                                    self_argument_sem_expr_idx: SemExprIdx(
                                        8,
                                    ),
                                    self_contract: Pure,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `ymin`,
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                    dispatch: FlyDynamicDispatch {
                                        indirections: FlyIndirections {
                                            initial_place: Leashed {
                                                place_idx: None,
                                            },
                                            indirections: [],
                                            final_place: Leashed {
                                                place_idx: None,
                                            },
                                        },
                                        signature: MethodFlySignature::MethodFn(
                                            MethodFnFlySignature {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::geom2d::BoundingBox(0)::ymin`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                self_value_parameter: FlyRitchieSimpleParameter {
                                                    contract: Pure,
                                                    ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Eth(
                                                            EthTerm(`BoundingBox`),
                                                        ),
                                                    },
                                                },
                                                parenate_parameters: [],
                                                return_ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Eth(
                                                        EthTerm(`f32`),
                                                    ),
                                                },
                                                instantiation: FlyInstantiation {
                                                    path: ItemPath::AssocItem(
                                                        AssocItemPath::TypeItem(
                                                            TypeItemPath(
                                                                `mnist_classifier::geom2d::BoundingBox(0)::ymin`,
                                                                TypeItemKind::MethodRitchie(
                                                                    RitchieItemKind::Fn,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    task_ty: Some(
                                                        EthTerm(`MnistTask`),
                                                    ),
                                                    env: MethodFn {
                                                        self_place: Leashed {
                                                            place_idx: None,
                                                        },
                                                    },
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    ritchie_parameter_argument_matches: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    9,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    opd: SemExprIdx(
                                        9,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`f32`),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    10,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::FunctionRitchieCall {
                                    function_sem_expr_idx: SemExprIdx(
                                        6,
                                    ),
                                    ritchie_ty_kind: RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    ritchie_parameter_argument_matches: [
                                        SemaRitchieArgument::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                            SemaSimpleArgument {
                                                argument_expr_idx: SemExprIdx(
                                                    10,
                                                ),
                                                coercion_outcome: Some(
                                                    ExpectCoercionOutcome {
                                                        coercion: Trivial(
                                                            TrivialFlyCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    },
                                                ),
                                                separator: CallListSeparator::None,
                                            },
                                        ),
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        25,
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    11,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        SemExprEntry {
                            data_result: Ok(
                                SemExprData::Block {
                                    stmts: SemStmtIdxRange(
                                        ArenaIdxRange(
                                            0..3,
                                        ),
                                    ),
                                },
                            ),
                            immediate_ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            expectation_idx_and_ty: Some(
                                (
                                    12,
                                    FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                },
            ),
            sem_stmt_arena: SemStmtArena(
                Arena {
                    data: [
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_pattern_sem_obelisk: LetVariableObelisk {
                                        syn_pattern_root: LetPatternSynExprRoot {
                                            syn_pattern_idx: 0,
                                        },
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon_token: None,
                                        ty_sem_expr_idx: None,
                                    },
                                    contract: Pure,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    initial_value_sem_expr_idx: SemExprIdx(
                                        1,
                                    ),
                                    coercion_outcome: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Require {
                                    require_token: RequireRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                    },
                                    condition: Other {
                                        sem_expr_idx: SemExprIdx(
                                            5,
                                        ),
                                        conversion: None,
                                    },
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Eth(
                                        EthTerm(`unit`),
                                    ),
                                },
                            ),
                        },
                        SemStmtEntry {
                            data_result: Ok(
                                SemStmtData::Eval {
                                    sem_expr_idx: SemExprIdx(
                                        11,
                                    ),
                                    outcome: Some(
                                        ExpectationOutcome::Coercion(
                                            ExpectCoercionOutcome {
                                                coercion: Trivial(
                                                    TrivialFlyCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                    eol_semicolon: None,
                                },
                            ),
                            ty_result: Ok(
                                FlyTerm {
                                    place: None,
                                    base: FlyTermBase::Hol(
                                        HolTerm(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
            sem_expr_roots: [
                (
                    12,
                    (
                        SemExprIdx(
                            12,
                        ),
                        SynExprRootKind::BlockExpr,
                    ),
                ),
            ],
            syn_pattern_expr_ty_infos: [
                PatternTypeInfo {
                    ty: Ok(
                        FlyTerm {
                            place: None,
                            base: Eth(
                                ItemPath(
                                    TypeOntology(
                                        TypePath(
                                            ItemPathId(
                                                Id {
                                                    value: 32,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                },
            ],
            syn_pattern_symbol_ty_infos: ArenaMap {
                data: [
                    Some(
                        PatternSymbolTypeInfo {
                            ty: Ok(
                                FlyTerm {
                                    place: None,
                                    base: Eth(
                                        ItemPath(
                                            TypeOntology(
                                                TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 32,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
            sem_expr_terms: [
                (
                    SemExprIdx(
                        4,
                    ),
                    Ok(
                        FlyTerm {
                            place: None,
                            base: FlyTermBase::Eth(
                                EthTerm(`0.0f32`),
                            ),
                        },
                    ),
                ),
            ],
            symbol_tys: SymbolMap {
                inherited_syn_symbol_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                StackPure {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Leash ConcaveComponent`),
                            ),
                        },
                    ),
                ],
                current_variable_map: [
                    SymbolType(
                        FlyTerm {
                            place: Some(
                                ImmutableOnStack {
                                    place: Idx(
                                        PlaceIdx(0),
                                    ),
                                },
                            ),
                            base: FlyTermBase::Eth(
                                EthTerm(`Vector2d`),
                            ),
                        },
                    ),
                ],
            },
            symbol_terms: SymbolMap {
                inherited_syn_symbol_map: [],
                current_variable_map: [],
            },
            fly_term_region: FlyTermRegion {
                terms: FlyTerms {
                    sol_terms: SolTerms {
                        entries: [],
                    },
                    hol_terms: HolTerms {
                        entries: [
                            HolTermEntry {
                                data: HolTermData::Hole {
                                    hole_source: HoleSource::Expr(
                                        6,
                                    ),
                                    hole_kind: ImplicitType,
                                    fill: Some(
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    ),
                                    constraints: [
                                        HoleConstraint::CoercibleFrom {
                                            target: FlyTerm {
                                                place: None,
                                                base: FlyTermBase::Eth(
                                                    EthTerm(`f32`),
                                                ),
                                            },
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`f32`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::TypeOntology {
                                    path: TypePath(`core::option::Option`, `Enum`),
                                    refined_path: Left(
                                        PreludeTypePath::Option,
                                    ),
                                    arguments: [
                                        FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`Option f32`),
                                ),
                            },
                            HolTermEntry {
                                data: HolTermData::Ritchie {
                                    ritchie_kind: RitchieKind::Type(
                                        RitchieTypeKind::Item(
                                            RitchieItemKind::Fn,
                                        ),
                                    ),
                                    params: [
                                        FlyRitchieParameter::Simple(
                                            FlyRitchieSimpleParameter {
                                                contract: Move,
                                                ty: FlyTerm {
                                                    place: None,
                                                    base: FlyTermBase::Hol(
                                                        HolTerm(
                                                            0,
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    ],
                                    return_ty: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                },
                                resolve_progress: HolTermResolveProgressBuf::ResolvedEthereal(
                                    EthTerm(`fn((move  f32) -> Option f32`),
                                ),
                            },
                        ],
                        first_unresolved_term_idx: 1,
                    },
                },
                expectations: Expectations {
                    arena: Arena {
                        data: [
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 0,
                                    src: ExpectationSource {
                                        syn_expr_idx: 0,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 1,
                                    src: ExpectationSource {
                                        syn_expr_idx: 1,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 2,
                                    src: ExpectationSource {
                                        syn_expr_idx: 2,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Vector2d`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 3,
                                    src: ExpectationSource {
                                        syn_expr_idx: 3,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Pure,
                                        ty_expected: FlyTerm {
                                            place: Some(
                                                ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            ),
                                            base: FlyTermBase::Eth(
                                                EthTerm(`f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 4,
                                    src: ExpectationSource {
                                        syn_expr_idx: 4,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Compterm,
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Compterm,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::ConditionType(
                                    ExpectConditionType,
                                ),
                                state: ExpectationState {
                                    idx: 5,
                                    src: ExpectationSource {
                                        syn_expr_idx: 5,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`bool`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::ConditionType(
                                                ExpectConditionTypeOutcome {
                                                    conversion: None,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::EqsFunctionType(
                                    ExpectEqsFunctionType {
                                        final_destination: FinalDestination::TypeOntology,
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 6,
                                    src: ExpectationSource {
                                        syn_expr_idx: 6,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                2,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::EqsFunctionCallType(
                                                ExpectEqsFunctionTypeOutcome {
                                                    return_ty: FlyTerm {
                                                        place: None,
                                                        base: FlyTermBase::Hol(
                                                            HolTerm(
                                                                1,
                                                            ),
                                                        ),
                                                    },
                                                    variant: ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                                                        ritchie_ty_kind: RitchieTypeKind::Item(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                        parameter_contracted_tys: [
                                                            FlyRitchieParameter::Simple(
                                                                FlyRitchieSimpleParameter {
                                                                    contract: Move,
                                                                    ty: FlyTerm {
                                                                        place: None,
                                                                        base: FlyTermBase::Hol(
                                                                            HolTerm(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                    },
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 7,
                                    src: ExpectationSource {
                                        syn_expr_idx: 7,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`Leash ConcaveComponent`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 8,
                                    src: ExpectationSource {
                                        syn_expr_idx: 8,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: Some(
                                            Leashed {
                                                place_idx: None,
                                            },
                                        ),
                                        base: FlyTermBase::Eth(
                                            EthTerm(`BoundingBox`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::AnyOriginal(
                                    ExpectAnyOriginal,
                                ),
                                state: ExpectationState {
                                    idx: 9,
                                    src: ExpectationSource {
                                        syn_expr_idx: 9,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::AnyOriginal(
                                                ExpectAnyOriginalOutcome,
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Hol(
                                                HolTerm(
                                                    0,
                                                ),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 10,
                                    src: ExpectationSource {
                                        syn_expr_idx: 10,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Eth(
                                            EthTerm(`f32`),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Option f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 11,
                                    src: ExpectationSource {
                                        syn_expr_idx: 11,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                            FlyTermExpectationEntry {
                                expectation: Expectation::Coercion(
                                    ExpectCoercion {
                                        contract: Move,
                                        ty_expected: FlyTerm {
                                            place: None,
                                            base: FlyTermBase::Eth(
                                                EthTerm(`Option f32`),
                                            ),
                                        },
                                    },
                                ),
                                state: ExpectationState {
                                    idx: 12,
                                    src: ExpectationSource {
                                        syn_expr_idx: 12,
                                        kind: Expr,
                                    },
                                    expectee: FlyTerm {
                                        place: None,
                                        base: FlyTermBase::Hol(
                                            HolTerm(
                                                1,
                                            ),
                                        ),
                                    },
                                    resolve_progress: ExpectationProgress::Resolved(
                                        Ok(
                                            ExpectationOutcome::Coercion(
                                                ExpectCoercionOutcome {
                                                    coercion: Trivial(
                                                        TrivialFlyCoercion {
                                                            expectee_quary: Transient,
                                                        },
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ],
                    },
                    first_unresolved_expectation: 0,
                },
            },
            return_ty: Some(
                EthTerm(`Option f32`),
            ),
            self_ty: None,
        },
    },
]
```