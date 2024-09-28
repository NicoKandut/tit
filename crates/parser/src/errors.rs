pub type LanguageError = tree_sitter::LanguageError;

#[derive(Debug, Clone)]
pub enum ParsingError {
    TreeSitterError,
    Utf8Error(std::str::Utf8Error)
}

#[derive(Debug, Clone)]
pub struct TreeIteratingError(pub &'static str);

#[derive(Debug)]
pub enum RenderingError {
    TreeIteratingError(TreeIteratingError),
    IoError(std::io::Error),
}
