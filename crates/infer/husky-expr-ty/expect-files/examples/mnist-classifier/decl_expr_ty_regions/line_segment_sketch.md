[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Some(
                    Ok(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
            },
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Some(
                    Ok(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
            },
            TypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: None,
            },
            TypeInfo {
                ty_result: Err(
                    Derived(
                        ApplicationArgumentTypeNotInferred,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: None,
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Some(
                    Ok(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Some(
                    Ok(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Some(
                    Ok(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Some(
                    Ok(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: None,
            },
            TypeInfo {
                ty_result: Err(
                    Derived(
                        ApplicationArgumentTypeNotInferred,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: None,
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::ImplBlock(
                ImplBlockId {
                    module_path: `mnist_classifier::line_segment_sketch`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: None,
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        },
                    },
                    ident: `new`,
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: None,
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        },
                    },
                    ident: `displacement`,
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Some(
                    Ok(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::ImplBlock(
                ImplBlockId {
                    module_path: `mnist_classifier::line_segment_sketch`,
                    impl_block_kind: ImplBlockKind::Type {
                        ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    },
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Some(
                    Ok(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        },
                    },
                    ident: `concave_components`,
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: None,
            },
            TypeInfo {
                ty_result: Err(
                    Derived(
                        ApplicationArgumentTypeNotInferred,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: None,
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        },
                    },
                    ident: `bounding_box`,
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Err(
                    Derived(
                        EntityTypeError,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: None,
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::AssociatedItem(
                AssociatedItemId {
                    impl_block_id: ImplBlockId {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        },
                    },
                    ident: `new`,
                },
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Some(
                    Ok(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
]