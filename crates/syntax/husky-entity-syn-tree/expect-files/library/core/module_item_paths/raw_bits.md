Ok(
    [
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::raw_bits::r32`, `Extern`),
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath {
                    module_path: `core::raw_bits`,
                    ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                    disambiguator: 0,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::raw_bits`,
                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `last_bits`,
                    item_kind: MethodFn,
                },
            ),
        ),
    ],
)