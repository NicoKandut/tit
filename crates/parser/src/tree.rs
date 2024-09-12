use crate::errors::{ChangeDetectionError, ParsingError};
use core::change::Change;
use std::collections::HashMap;
use std::str::Utf8Error;
use tree_sitter::{Node, Tree};

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

    pub fn detect_changes(
        &self,
        other: &TitTree,
    ) -> Result<impl Iterator<Item = Change>, ChangeDetectionError> {
        detect_changes_in_nodes(
            Some(&self.tree.root_node()),
            Some(&other.tree.root_node()),
            &self.value_map,
            &other.value_map, 
            &mut vec![],
        )
    }

    pub fn apply_changes<I>(&mut self, changes: I)
    where
        I: IntoIterator<Item = Change>,
    {
        for change in changes {
            // TODO: process each change
        }
    }
}

fn construct_value_map(
    node: &Node,
    source: &[u8],
    value_map: &mut HashMap<usize, String>,
) -> Result<(), Utf8Error> {
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

fn detect_changes_in_nodes(
    n1: Option<&Node>,
    n2: Option<&Node>,
    value_map1: &HashMap<usize, String>,
    value_map2: &HashMap<usize, String>,
    path: &mut Vec<u16>,
) -> Result<impl Iterator<Item = Change>, ChangeDetectionError> {
    let mut differences = Vec::new();

    match (n1, n2) {
        (Some(n1), Some(n2)) => {
            if n1.kind() != n2.kind() {
                differences.push(Change::KindUpdate {
                    path: path.to_vec(),
                    update: n2.kind().to_string(),
                });
            }

            if n1.child_count() == 0
                && n2.child_count() == 0
                && value_map1.get(&n1.id()) != value_map2.get(&n2.id())
            {
                differences.push(Change::ValueUpdate {
                    path: path.to_vec(),
                    update: match value_map2.get(&n2.id()) {
                        Some(value) => value.to_string(),
                        None => Err(ChangeDetectionError::NoValueForNode {
                            path: path.to_vec(),
                            node: n2.kind().to_string(),
                        })?
                    }
                });
            }

            for (index, (child1, child2)) in zip_named_children_with_none(&n1, &n2).enumerate() {
                path.push(index as u16);
                let child_diffs = detect_changes_in_nodes(
                    child1.as_ref(),
                    child2.as_ref(),
                    value_map1,
                    value_map2,
                    path,
                )?;
                differences.extend(child_diffs);
                path.pop();
            }
        }
        (Some(_), None) => {
            differences.push(Change::Deletion {
                path: path.to_vec(),
            });
        }
        (None, Some(n2)) => {
            differences.push(Change::Addition {
                path: path.to_vec(),
                kind: n2.kind().to_string(),
                value: value_map2.get(&n2.id()).cloned(),
            });
            for (index, child) in n2.named_children(&mut n2.walk()).enumerate() {
                path.push(index as u16);
                let child_diffs =
                    detect_changes_in_nodes(None, Some(&child), value_map1, value_map2, path)?;
                differences.extend(child_diffs);
                path.pop();
            }
        }
        (None, None) => {}
    }

    Ok(differences.into_iter())
}

fn zip_named_children_with_none<'a>(
    node1: &'a Node<'a>,
    node2: &'a Node<'a>,
) -> impl Iterator<Item = (Option<Node<'a>>, Option<Node<'a>>)> + 'a {
    let mut cursor1 = node1.walk();
    let mut cursor2 = node2.walk();

    let children1: Vec<_> = node1.named_children(&mut cursor1).map(Some).collect();
    let children2: Vec<_> = node2.named_children(&mut cursor2).map(Some).collect();

    let max_len = children1.len().max(children2.len());

    children1
        .into_iter()
        .chain(std::iter::repeat_with(|| None))
        .zip(children2.into_iter().chain(std::iter::repeat_with(|| None)))
        .take(max_len)
}
