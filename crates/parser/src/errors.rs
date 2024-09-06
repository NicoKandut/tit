pub type LanguageError = tree_sitter::LanguageError;

#[derive(Debug, Clone)]
pub enum ParsingError {
    TreeSitterError,
    Utf8Error(std::str::Utf8Error),
}

