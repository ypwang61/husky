use word::Keyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeKind {
    Module,
    Value,
    Type,
    Trait,
    Func,
}

impl ScopeKind {
    pub(crate) fn new(keyword: Keyword) -> Option<ScopeKind> {
        match keyword {
            Keyword::Use | Keyword::Stmt(_) => None,
            Keyword::Mod => Some(ScopeKind::Module),
            Keyword::Func(_) => Some(ScopeKind::Func),
            Keyword::Type(_) => Some(ScopeKind::Type),
        }
    }
}
