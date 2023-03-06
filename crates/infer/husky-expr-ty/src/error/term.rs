use super::*;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExprTermError {
    #[error("original expr term error: {0}")]
    Original(#[from] OriginalExprTermError),
    #[error("derived expr term error: {0}")]
    Derived(#[from] DerivedExprTermError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalExprTermError {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedExprTermError {
    #[error("expr error")]
    ExprError,
    #[error("todo")]
    Todo,
}

pub type ExprTermResult<T> = Result<T, ExprTermError>;