mod kinds;
mod mutable_node;
mod mutable_node_change;

use std::fmt;
use std::str::Utf8Error;

use indextree::{Arena, NodeId};
use tree_sitter::{Node, Tree};

use core::change::{Change, ChangeKind};

use crate::errors::ParsingError;
use crate::tree::kinds::{significant_unnamed_kinds, Kinds, insignificant_named_kinds};
use crate::tree::mutable_node::MutableNode;
use crate::tree::mutable_node_change::{MutableNodeChange, MutableNodeChangeKind};

pub struct TitTree {
    arena: Arena<MutableNode>,
    root: NodeId
}

impl TitTree {
    pub fn new(tree: Tree, source: &[u8]) -> Result<Self, ParsingError> {
        let mut arena = Arena::new();
        let root = arena.new_node(MutableNode {
            kind: tree.root_node().kind().to_string(),
            value: None,
        });

        let significant_unnamed_kinds = significant_unnamed_kinds(tree.language());
        let insignificant_named_kinds = insignificant_named_kinds(tree.language());
        let result = construct_arena(
            &tree.root_node(),
            source,
            &mut arena,
            &root,
            &significant_unnamed_kinds,
            &insignificant_named_kinds,
        );
        match result {
            Ok(_) => Ok(Self {
                arena,
                root
            }),
            Err(e) => Err(ParsingError::Utf8Error(e)),
        }
    }

    pub fn detect_changes(&self, other: &TitTree) -> Vec<Change> {
        detect_changes_in_nodes(
            Some(&self.root),
            Some(&other.root),
            &self.arena,
            &other.arena,
            &mut vec![0],
        )
    }

    pub fn apply_changes(&mut self, changes: &Vec<Change>) {
        let change_refs: Vec<&Change> = changes.iter().collect();
        let changed_nodes = construct_changed_nodes(self.root, &change_refs, &mut self.arena, 0, 0);

        for node_change in changed_nodes {
            let node = self
                .arena
                .get_mut(node_change.node)
                .expect("Node should exist")
                .get_mut();
            match node_change.kind {
                MutableNodeChangeKind::KindUpdate { kind, value } => {
                    node.kind = kind.to_string();
                    node.value = value.clone();
                }
                MutableNodeChangeKind::ValueUpdate(value) => {
                    node.value = Some(value.to_string());
                }
                MutableNodeChangeKind::Addition { parent } => {
                    parent.append(node_change.node, &mut self.arena);
                }
                MutableNodeChangeKind::Deletion => {
                    node_change.node.remove_subtree(&mut self.arena);
                }
            }
        }
    }
}

impl fmt::Debug for TitTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = self.root.debug_pretty_print(&self.arena);
        write!(f, "{:?}", printable)
    }
}

impl PartialEq for TitTree {
    fn eq(&self, other: &Self) -> bool {
        nodes_equal(&self.root, &other.root, &self.arena, &other.arena)
    }
}

fn construct_arena(
    node: &Node,
    source: &[u8],
    arena: &mut Arena<MutableNode>,
    arena_node: &NodeId,
    significant_unnamed_kinds: &Kinds,
    insignificant_named_kinds: &Kinds,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if !child.is_named() && !significant_unnamed_kinds.contains(child.kind()) {
            continue;
        }
        
        let new_arena_node = if !child.is_named() || !insignificant_named_kinds.contains(child.kind()) {
            let child_node = MutableNode {
                kind: child.kind().to_string(),
                value: if child.child_count() == 0 {
                    Some(child.utf8_text(source)?.to_string())
                } else {
                    None
                },
            };
            &arena_node.append_value(child_node, arena)
        }
        else {
            arena_node
        };
        
        construct_arena(&child, source, arena, &new_arena_node, significant_unnamed_kinds, insignificant_named_kinds)?;
    }

    Ok(())
}

fn detect_changes_in_nodes(
    n1: Option<&NodeId>,
    n2: Option<&NodeId>,
    arena1: &Arena<MutableNode>,
    arena2: &Arena<MutableNode>,
    path: &mut Vec<u16>,
) -> Vec<Change> {
    let mut differences = Vec::new();

    match (n1, n2) {
        (Some(n1), Some(n2)) => {
            let node1 = arena1
                .get(*n1)
                .expect("Node 1 should exist in arena 1")
                .get();
            let node2 = arena2
                .get(*n2)
                .expect("Node 2 should exist in arena 2")
                .get();

            if node1.kind != node2.kind {
                differences.push(Change {
                    path: path.to_vec(),
                    kind: ChangeKind::KindUpdate { kind: node2.kind.to_string(), value: node2.value.clone() },
                });
            }

            if n1.children(arena1).peekable().peek().is_none()
                && n2.children(arena2).peekable().peek().is_none()
                && node1.value != node2.value
            {
                differences.push(Change {
                    path: path.to_vec(),
                    kind: ChangeKind::ValueUpdate(
                        node2.value.clone().expect("Node 2 should have a value"),
                    ),
                });
            }

            for (index, (child1, child2)) in
                zip_children_with_none(n1, n2, arena1, arena2).enumerate()
            {
                path.push(index as u16);
                let child_diffs =
                    detect_changes_in_nodes(child1.as_ref(), child2.as_ref(), arena1, arena2, path);
                differences.extend(child_diffs);
                path.pop();
            }
        }
        (Some(_), None) => {
            differences.push(Change {
                path: path.to_vec(),
                kind: ChangeKind::Deletion,
            });
        }
        (None, Some(n2)) => {
            let node2 = arena2
                .get(*n2)
                .expect("Node 2 should exist in arena 2")
                .get();

            differences.push(Change {
                path: path.to_vec(),
                kind: ChangeKind::Addition {
                    kind: node2.kind.to_string(),
                    value: node2.value.clone(),
                },
            });

            for (index, child) in n2.children(arena2).enumerate() {
                path.push(index as u16);
                let child_diffs = detect_changes_in_nodes(None, Some(&child), arena1, arena2, path);
                differences.extend(child_diffs);
                path.pop();
            }
        }
        (None, None) => {}
    }

    differences
}

fn construct_changed_nodes<'a>(
    node: NodeId,
    changes: &Vec<&'a Change>,
    arena: &mut Arena<MutableNode>,
    level: usize,
    offset: u16,
) -> Vec<MutableNodeChange<'a>> {
    let mut node_changes = Vec::new();

    let relevant_changes: Vec<_> = changes
        .iter()
        .filter(|change| change.path.len() > level && change.path[level] == offset)
        .cloned()
        .collect();

    if relevant_changes.is_empty() {
        return node_changes;
    }

    let applicable_changes = relevant_changes
        .iter()
        .filter(|change| change.path.len() == level + 1);

    for change in applicable_changes {
        match &change.kind {
            ChangeKind::KindUpdate { kind, value} => {
                node_changes.push(MutableNodeChange {
                    kind: MutableNodeChangeKind::KindUpdate { kind, value },
                    node,
                });
            }
            ChangeKind::ValueUpdate(update) => {
                node_changes.push(MutableNodeChange {
                    kind: MutableNodeChangeKind::ValueUpdate(update),
                    node,
                });
            }
            ChangeKind::Deletion => {
                node_changes.push(MutableNodeChange {
                    kind: MutableNodeChangeKind::Deletion,
                    node,
                });
            }
            _ => panic!("Unexpected change kind"),
        }
    }

    let children: Vec<_> = node.children(arena).collect();
    for (index, child) in children.into_iter().enumerate() {
        let child_changes =
            construct_changed_nodes(child, &relevant_changes, arena, level + 1, index as u16);
        node_changes.extend(child_changes);
    }
    
    let additions = relevant_changes
        .iter()
        .filter(|change| change.path.len() == level + 2 && matches!(change.kind, ChangeKind::Addition { .. }));
    
    for addition in additions {
        if let ChangeKind::Addition { kind, value } = &addition.kind {
            let child_node = MutableNode {
                kind: kind.to_string(),
                value: value.clone(),
            };
            let new_node = arena.new_node(child_node);
            node_changes.push(MutableNodeChange {
                kind: MutableNodeChangeKind::Addition { parent: node },
                node: new_node,
            });

            let new_node_offset = *addition.path.last().expect("Path should not be empty");
            let addition_children = relevant_changes
                .iter()
                .filter(|change| change.path.len() >= level + 3 && change.path[level + 1] == new_node_offset)
                .cloned()
                .collect();

            node_changes.extend(construct_changed_nodes(new_node, &addition_children, arena, level + 1, new_node_offset));
        }
    }

    node_changes
}

fn zip_children_with_none(
    n1: &NodeId,
    n2: &NodeId,
    arena1: &Arena<MutableNode>,
    arena2: &Arena<MutableNode>,
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

fn nodes_equal(
    n1: &NodeId,
    n2: &NodeId,
    arena1: &Arena<MutableNode>,
    arena2: &Arena<MutableNode>,
) -> bool {
    let node1 = arena1
        .get(*n1)
        .expect("Node 1 should exist in arena 1")
        .get();
    let node2 = arena2
        .get(*n2)
        .expect("Node 2 should exist in arena 2")
        .get();

    if node1 != node2 {
        return false;
    }

    let children1: Vec<_> = n1.children(arena1).collect();
    let children2: Vec<_> = n2.children(arena2).collect();

    if children1.len() != children2.len() {
        return false;
    }

    for (child1, child2) in children1.into_iter().zip(children2.into_iter()) {
        if !nodes_equal(&child1, &child2, arena1, arena2) {
            return false;
        }
    }

    true
}
