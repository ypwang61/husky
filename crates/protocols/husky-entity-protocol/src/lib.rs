#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityClass {
    Module,
    Type,
    MajorFunctionRitchie,
    TypeAlias,
    TypeVar,
    Val,
    Trait,
    TypeVariant,
    MethodRitchie,
    AssocRitchie,
    MemoizedField,
    AssocVal,
    AssocType,
    ImplBlock,
    Attr,
    Formal,
    AssocDef,
    Compterm,
    Script,
    StaticMut,
    StaticVar,
}
