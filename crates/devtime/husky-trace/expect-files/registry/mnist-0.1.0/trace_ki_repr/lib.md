```rust
[
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist::input`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 1,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                },
            ),
        },
        Some(
            KiRepr {
                val_domain_repr: Omni,
                opn: ValOpn::Linkage(
                    Linkage {
                        data: LinkageData::MajorVal {
                            path: FugitivePath(`mnist::input`, `Val`),
                            instantiation: LinInstantiation {
                                symbol_resolutions: [],
                                separator: None,
                            },
                        },
                    },
                ),
                arguments: [],
                source: KiReprSource::ValItem(
                    FugitivePath(`mnist::input`, `Val`),
                ),
                caching_class: ValItem,
            },
        ),
    ),
]
```