[
    (
        TypePath(`core::logic::Prop`, `Alien`),
        Ok(
            Application(
                TermApplication {
                    function: Category(
                        Sort,
                    ),
                    argument: Universe(
                        TermUniverse(
                            1,
                        ),
                    ),
                },
            ),
        ),
    ),
    (
        TypePath(`core::logic::LogicAnd`, `Structure`),
        Ok(
            Curry(
                TermCurry {
                    variance: Independent,
                    x: Application(
                        TermApplication {
                            function: Category(
                                Sort,
                            ),
                            argument: Universe(
                                TermUniverse(
                                    1,
                                ),
                            ),
                        },
                    ),
                    y: Curry(
                        TermCurry {
                            variance: Independent,
                            x: Application(
                                TermApplication {
                                    function: Category(
                                        Sort,
                                    ),
                                    argument: Universe(
                                        TermUniverse(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            y: Application(
                                TermApplication {
                                    function: Category(
                                        Sort,
                                    ),
                                    argument: Universe(
                                        TermUniverse(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                },
            ),
        ),
    ),
    (
        TypePath(`core::logic::LogicOr`, `Inductive`),
        Ok(
            Curry(
                TermCurry {
                    variance: Independent,
                    x: Application(
                        TermApplication {
                            function: Category(
                                Sort,
                            ),
                            argument: Universe(
                                TermUniverse(
                                    1,
                                ),
                            ),
                        },
                    ),
                    y: Curry(
                        TermCurry {
                            variance: Independent,
                            x: Application(
                                TermApplication {
                                    function: Category(
                                        Sort,
                                    ),
                                    argument: Universe(
                                        TermUniverse(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                            y: Application(
                                TermApplication {
                                    function: Category(
                                        Sort,
                                    ),
                                    argument: Universe(
                                        TermUniverse(
                                            1,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                },
            ),
        ),
    ),
]