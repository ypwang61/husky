```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist::task::MnistTask`, `Extern`),
            ),
        ),
        None,
    ),
    (
        ItemPath::Attr(
            Room32,
            AttrItemPath(`mnist::task::MnistTask::@task(0)`),
        ),
        None,
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(`mnist::task::MnistTask(0)`),
            ),
        ),
        None,
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(
                    `mnist::task::MnistTask(0)::new`,
                    TypeItemKind::AssocRitchie(
                        RitchieItemKind::Fn,
                    ),
                ),
            ),
        ),
        None,
    ),
]
```