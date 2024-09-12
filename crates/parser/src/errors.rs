pub type LanguageError = tree_sitter::LanguageError;

#[derive(Debug, Clone)]
pub enum ParsingError {
    TreeSitterError,
    Utf8Error(std::str::Utf8Error),
}

#[derive(Debug, Clone)]
pub enum ChangeDetectionError {
    NoValueForNode { path: Vec<u16>, node: String },
}

