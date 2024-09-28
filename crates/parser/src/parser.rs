use std::str::Utf8Error;
use indextree::{Arena, NodeId};
use tree_sitter::{Language, Parser};
use kern::{Node, TitTree};
use crate::errors::{LanguageError, ParsingError};
use crate::kinds::{insignificant_named_kinds, Kinds, significant_unnamed_kinds};

pub struct TitParser {
    parser: Parser
}

impl TitParser {
    pub fn new(language: Language) -> Result<Self, LanguageError> {
        let mut parser = Parser::new();
        match parser.set_language(&language) {
            Ok(_) => Ok(Self { parser }),
            Err(e) => Err(e),
        }
    }
    
    pub fn parse(&mut self, source: impl AsRef<[u8]>) -> Result<TitTree, ParsingError> {
        let source_ref = source.as_ref();
        
        match self.parser.parse(source_ref, None) {
            Some(tree) => {
                let root_node = tree.root_node();
                let mut arena = Arena::new();
                let root = arena.new_node(Node {
                    kind: root_node.kind().to_string(),
                    value: None,
                    role: None,
                });
                
                construct_arena(
                    &root_node,
                    source_ref,
                    &mut arena,
                    &root,
                    &significant_unnamed_kinds(&self.parser.language().expect("Language should be set set")),
                    &insignificant_named_kinds(&self.parser.language().expect("Language should be set set")),
                    None,
                ).map_err(ParsingError::Utf8Error)?;
                
                Ok(TitTree::new(arena, root))
            },
            None => Err(ParsingError::TreeSitterError),
        }
    }
}

fn construct_arena(
    node: &tree_sitter::Node,
    source: &[u8],
    arena: &mut Arena<Node>,
    arena_node: &NodeId,
    significant_unnamed_kinds: &Kinds,
    insignificant_named_kinds: &Kinds,
    passed_field: Option<&str>,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for (index, child) in node.children(&mut cursor).enumerate() {
        let field = node.field_name_for_child(index as u32);
        if field.is_some() && passed_field.is_some() {
            panic!("Field should not be set twice");
        }

        let field = field.or(passed_field);

        if !child.is_named() && !significant_unnamed_kinds.contains(child.kind()) {
            continue;
        }

        let (new_arena_node, field) =
            if !child.is_named() || !insignificant_named_kinds.contains(child.kind()) {
                let child_node = Node {
                    kind: child.kind().to_string(),
                    value: if child.child_count() == 0 {
                        Some(child.utf8_text(source)?.to_string())
                    } else {
                        None
                    },
                    role: field.map(|f| f.to_string()),
                };
                (&arena_node.append_value(child_node, arena), None)
            } else {
                (arena_node, field)
            };

        construct_arena(
            &child,
            source,
            arena,
            &new_arena_node,
            significant_unnamed_kinds,
            insignificant_named_kinds,
            field,
        )?;
    }

    Ok(())
}