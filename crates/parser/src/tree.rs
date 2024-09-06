use std::collections::HashMap;
use std::str::Utf8Error;
use tree_sitter::{Node, Tree};
use crate::errors::ParsingError;

pub struct TitTree {
    tree: Tree,
    value_map: HashMap<usize, String>,
}

impl TitTree {
    pub fn new(tree: Tree, source: &[u8]) -> Result<Self, ParsingError> {
        let mut value_map: HashMap<usize, String> = HashMap::new();
        let result = construct_value_map(&tree.root_node(), source, &mut value_map);
        match result {
            Ok(_) => Ok(Self { tree, value_map }),
            Err(e) => Err(ParsingError::Utf8Error(e)),
        }
    }
}

fn construct_value_map(node: &Node, source: &[u8], value_map: &mut HashMap<usize, String>) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.child_count() == 0 {
            let value = child.utf8_text(source)?.to_string();
            value_map.insert(child.id(), value);
        }
        construct_value_map(&child, source, value_map)?;
    }
    
    Ok(())
}