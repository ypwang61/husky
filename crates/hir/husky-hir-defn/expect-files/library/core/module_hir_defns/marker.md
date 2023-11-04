[
    HirDefn::MajorItem(
        MajorItemHirDefn::Trait(
            TraitHirDefn {
                path: TraitPath(`core::marker::Copy`),
                hir_decl: TraitHirDecl {
                    path: TraitPath(`core::marker::Copy`),
                    template_parameters: HirTemplateParameters {
                        data: [
                            HirTemplateParameter {
                                symbol: Type(
                                    SelfType,
                                ),
                                traits: [],
                            },
                        ],
                    },
                },
            },
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Trait(
            TraitHirDefn {
                path: TraitPath(`core::marker::Sized`),
                hir_decl: TraitHirDecl {
                    path: TraitPath(`core::marker::Sized`),
                    template_parameters: HirTemplateParameters {
                        data: [
                            HirTemplateParameter {
                                symbol: Type(
                                    SelfType,
                                ),
                                traits: [],
                            },
                        ],
                    },
                },
            },
        ),
    ),
]