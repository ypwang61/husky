Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::raw_contour`,
        module_symbols: [
            ModuleItem {
                ident: `RawContour`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 217,
                path: TypePath(`mnist_classifier::raw_contour::RawContour, Struct`),
            },
            ModuleItem {
                ident: `Direction`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 219,
                path: TypePath(`mnist_classifier::raw_contour::Direction, Enum`),
            },
            ModuleItem {
                ident: `get_pixel_pair`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 224,
                path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair, Function`),
            },
            ModuleItem {
                ident: `get_pixel_to_the_left`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 225,
                path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left, Function`),
            },
            ModuleItem {
                ident: `get_pixel_to_the_right`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 226,
                path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right, Function`),
            },
            ModuleItem {
                ident: `get_inward_direction`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 227,
                path: FormPath(`mnist_classifier::raw_contour::get_inward_direction, Function`),
            },
            ModuleItem {
                ident: `get_angle_change`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 228,
                path: FormPath(`mnist_classifier::raw_contour::get_angle_change, Function`),
            },
            ModuleItem {
                ident: `get_outward_direction`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 229,
                path: FormPath(`mnist_classifier::raw_contour::get_outward_direction, Function`),
            },
            ModuleItem {
                ident: `StreakCache`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 230,
                path: TypePath(`mnist_classifier::raw_contour::StreakCache, Struct`),
            },
            ModuleItem {
                ident: `get_concave_middle_point`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 231,
                path: FormPath(`mnist_classifier::raw_contour::get_concave_middle_point, Function`),
            },
            ModuleItem {
                ident: `find_raw_contours`,
                accessibility: PubicUnder(
                    `mnist_classifier::raw_contour`,
                ),
                ast_idx: 232,
                path: FormPath(`mnist_classifier::raw_contour::find_raw_contours, Function`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                EntityUseTracker {
                    ast_idx: 213,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 33,
                            },
                        ),
                    ),
                    ident: Identifier(
                        Word(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    use_expr_idx: 2,
                    parent: None,
                    state: Unresolved,
                },
                EntityUseTracker {
                    ast_idx: 214,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 33,
                            },
                        ),
                    ),
                    ident: Identifier(
                        Word(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    use_expr_idx: 5,
                    parent: None,
                    state: Unresolved,
                },
                EntityUseTracker {
                    ast_idx: 215,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 33,
                            },
                        ),
                    ),
                    ident: Identifier(
                        Word(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    use_expr_idx: 8,
                    parent: None,
                    state: Unresolved,
                },
                EntityUseTracker {
                    ast_idx: 216,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 33,
                            },
                        ),
                    ),
                    ident: Identifier(
                        Word(
                            Id {
                                value: 80,
                            },
                        ),
                    ),
                    use_expr_idx: 12,
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)