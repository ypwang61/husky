use crate::mode::LxMode;
use latex_vfs::path::LxFilePath;

pub trait IsLxInput<'a>: Copy {
    fn content(self) -> &'a str;
    fn root_mode(self) -> LxMode;
    fn file_path(self) -> LxFilePath;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxDocumentInput<'a> {
    pub file_path: LxFilePath,
    pub content: &'a str,
}

impl<'a> IsLxInput<'a> for LxDocumentInput<'a> {
    fn file_path(self) -> LxFilePath {
        self.file_path
    }

    fn content(self) -> &'a str {
        self.content
    }

    fn root_mode(self) -> LxMode {
        LxMode::Root
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxDocumentBodyInput<'a> {
    pub file_path: LxFilePath,
    pub content: &'a str,
}

impl<'a> IsLxInput<'a> for LxDocumentBodyInput<'a> {
    fn file_path(self) -> LxFilePath {
        self.file_path
    }

    fn content(self) -> &'a str {
        self.content
    }

    fn root_mode(self) -> LxMode {
        LxMode::Rose
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxPageInput<'a> {
    pub file_path: LxFilePath,
    pub content: &'a str,
}

impl<'a> IsLxInput<'a> for LxPageInput<'a> {
    fn file_path(self) -> LxFilePath {
        self.file_path
    }

    fn content(self) -> &'a str {
        self.content
    }

    fn root_mode(self) -> LxMode {
        LxMode::Rose
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxFormulaInput<'a> {
    pub file_path: LxFilePath,
    pub content: &'a str,
}

impl<'a> IsLxInput<'a> for LxFormulaInput<'a> {
    fn file_path(self) -> LxFilePath {
        self.file_path
    }

    fn content(self) -> &'a str {
        self.content
    }

    fn root_mode(self) -> LxMode {
        LxMode::Math
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LxLispInput<'a> {
    pub file_path: LxFilePath,
    pub content: &'a str,
}

impl<'a> IsLxInput<'a> for LxLispInput<'a> {
    fn file_path(self) -> LxFilePath {
        self.file_path
    }

    fn content(self) -> &'a str {
        self.content
    }
    fn root_mode(self) -> LxMode {
        LxMode::Lisp
    }
}
