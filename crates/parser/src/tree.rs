use std::str::Utf8Error;

use indextree::{Arena, NodeId};
use tree_sitter::{Node, Tree};

use core::change::Change;

use crate::errors::{ChangeDetectionError, ParsingError};

#[derive(Debug, Clone)]
pub struct TitNode {
    pub kind: String,
    pub value: Option<String>,
}

pub struct TitTree {
    arena: Arena<TitNode>,
    root: NodeId,
}

impl TitTree {
    pub fn new(tree: Tree, source: &[u8]) -> Result<Self, ParsingError> {
        let mut arena = Arena::new();
        let root = arena.new_node(TitNode {
            kind: tree.root_node().kind().to_string(),
            value: None,
        });

        let result = construct_arena(&tree.root_node(), source, &mut arena, &root);
        match result {
            Ok(_) => Ok(Self { arena, root }),
            Err(e) => Err(ParsingError::Utf8Error(e)),
        }
    }

    pub fn detect_changes(
        &self,
        other: &TitTree,
    ) -> Result<impl Iterator<Item = Change>, ChangeDetectionError> {
        detect_changes_in_nodes(
            Some(&self.root),
            Some(&other.root),
            &self.arena,
            &other.arena,
            &mut vec![0],
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

    pub fn print(&self) {
        let printable = self.root.debug_pretty_print(&self.arena);
        println!("{:?}", printable);
    }
}

fn construct_arena(
    node: &Node,
    source: &[u8],
    arena: &mut Arena<TitNode>,
    arena_node: &NodeId,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        let child_node = TitNode {
            kind: child.kind().to_string(),
            value: if child.child_count() == 0 {
                Some(child.utf8_text(source)?.to_string())
            } else {
                None
            },
        };
        let arena_child_node = arena_node.append_value(child_node, arena);
        construct_arena(&child, source, arena, &arena_child_node)?;
    }

    Ok(())
}

fn detect_changes_in_nodes(
    n1: Option<&NodeId>,
    n2: Option<&NodeId>,
    arena1: &Arena<TitNode>,
    arena2: &Arena<TitNode>,
    path: &mut Vec<u16>,
) -> Result<impl Iterator<Item = Change>, ChangeDetectionError> {
    let mut differences = Vec::new();

    match (n1, n2) {
        (Some(n1), Some(n2)) => {
            let node1 = arena1
                .get(*n1)
                .ok_or(ChangeDetectionError::NoNodeForPathError {
                    path: path.to_vec(),
                })?
                .get();
            let node2 = arena2
                .get(*n2)
                .ok_or(ChangeDetectionError::NoNodeForPathError {
                    path: path.to_vec(),
                })?
                .get();

            if node1.kind != node2.kind {
                differences.push(Change::KindUpdate {
                    path: path.to_vec(),
                    update: node2.kind.to_string(),
                });
            }

            if n1.children(arena1).peekable().peek().is_none()
                && n2.children(arena2).peekable().peek().is_none()
                && node1.value != node2.value
            {
                differences.push(Change::ValueUpdate {
                    path: path.to_vec(),
                    update: match node2.clone().value {
                        Some(value) => value.to_string(),
                        None => Err(ChangeDetectionError::NoValueForNodeError {
                            path: path.to_vec(),
                            node: node2.kind.to_string(),
                        })?,
                    },
                });
            }

            for (index, (child1, child2)) in
                zip_children_with_none(n1, n2, arena1, arena2).enumerate()
            {
                path.push(index as u16);
                let child_diffs = detect_changes_in_nodes(
                    child1.as_ref(),
                    child2.as_ref(),
                    arena1,
                    arena2,
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
            let node2 = arena2
                .get(*n2)
                .ok_or(ChangeDetectionError::NoNodeForPathError {
                    path: path.to_vec(),
                })?
                .get();
            differences.push(Change::Addition {
                path: path.to_vec(),
                kind: node2.kind.to_string(),
                value: node2.value.clone(),
            });

            for (index, child) in n2.children(arena2).enumerate() {
                path.push(index as u16);
                let child_diffs =
                    detect_changes_in_nodes(None, Some(&child), arena1, arena2, path)?;
                differences.extend(child_diffs);
                path.pop();
            }
        }
        (None, None) => {}
    }

    Ok(differences.into_iter())
}

fn zip_children_with_none(
    n1: &NodeId,
    n2: &NodeId,
    arena1: &Arena<TitNode>,
    arena2: &Arena<TitNode>,
) -> impl Iterator<Item = (Option<NodeId>, Option<NodeId>)> {
    let children1: Vec<_> = n1.children(arena1).map(Some).collect();
    let children2: Vec<_> = n2.children(arena2).map(Some).collect();

    let max_len = children1.len().max(children2.len());

    children1
        .into_iter()
        .chain(std::iter::repeat_with(|| None))
        .zip(children2.into_iter().chain(std::iter::repeat_with(|| None)))
        .take(max_len)
}
