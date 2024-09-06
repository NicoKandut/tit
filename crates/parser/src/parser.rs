use tree_sitter::{Language, Parser};
use crate::errors::{LanguageError, ParsingError};
use crate::tree::TitTree;

pub struct TitParser {
    parser: Parser
}

impl TitParser {
    pub fn new(language: &Language) -> Result<Self, LanguageError> {
        let mut parser = Parser::new();
        match parser.set_language(language) {
            Ok(_) => Ok(Self { parser }),
            Err(e) => Err(e),
        }
    }
    
    pub fn parse(&mut self, source: impl AsRef<[u8]>) -> Result<TitTree, ParsingError> {
        let source_ref = source.as_ref();
        
        match self.parser.parse(source_ref, None) {
            Some(tree) => TitTree::new(tree, source_ref),
            None => Err(ParsingError::TreeSitterError),
        }
    }
}