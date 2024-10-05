use std::io::{Read, Write};
use std::{fmt, fs};

use indextree::{Arena, NodeId};
use serde::{Deserialize, Serialize};

use crate::tree::node_change::NodeChange;
use crate::util::{from_serialized_bytes, to_serialized_bytes, BinaryFileRead, BinaryFileWrite};
use crate::{Change, Node, Path, TitError};

mod node_change;

#[derive(Serialize, Deserialize)]
pub struct TitTree {
    arena: Arena<Node>,
    root: NodeId,
}

impl Default for TitTree {
    fn default() -> Self {
        let mut arena = Arena::default();
        let root = arena.new_node(Node {
            kind: "".to_string(),
            value: None,
            role: None,
        });
        Self { arena, root }
    }
}

impl<P: AsRef<std::path::Path>> BinaryFileRead<P> for TitTree {
    fn read_from(path: P) -> Self {
        let mut compressed_bytes = vec![];
        fs::File::open(path)
            .expect("Failed to open tree file!")
            .read_to_end(&mut compressed_bytes)
            .expect("Failed to read tree file!");
        from_serialized_bytes(&compressed_bytes).expect("Failed to deserialize tree")
    }
}

impl<P: AsRef<std::path::Path>> BinaryFileWrite<P> for TitTree {
    fn write_to(&self, path: P) {
        let compressed_bytes = to_serialized_bytes(self).expect("Failed to serialize tree");
        fs::File::create(path)
            .expect("Failed to create file!")
            .write(&compressed_bytes)
            .expect("Failed to write tree!");
    }
}

impl TitTree {
    pub fn new(arena: Arena<Node>, root: NodeId) -> Self {
        TitTree { arena, root }
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
        let node_changes = construct_changed_nodes(self.root, &change_refs, &mut self.arena, 0, 0);

        for node_change in node_changes {
            let node = self
                .arena
                .get_mut(node_change.node_id())
                .expect("Node should exist")
                .get_mut();

            match node_change {
                NodeChange::Update(_, new_node) => {
                    node.kind = new_node.kind.to_string();
                    node.value = new_node.value.clone();
                    node.role = new_node.role.clone();
                }
                NodeChange::Addition(_, parent) => {
                    parent.append(node_change.node_id(), &mut self.arena);
                }
                NodeChange::Deletion(_) => {
                    node_change.node_id().remove_subtree(&mut self.arena);
                }
            }
        }
    }

    pub fn root(&self) -> Result<&indextree::Node<Node>, TitError<'static>> {
        self.arena
            .get(self.root)
            .ok_or(TitError("Root node not found", None))
    }

    pub fn children(
        &self,
        node: &indextree::Node<Node>,
    ) -> Result<impl Iterator<Item = &indextree::Node<Node>>, TitError<'static>> {
        let node_id = self
            .arena
            .get_node_id(node)
            .ok_or(TitError("Node not found", None))?;

        let children = node_id
            .children(&self.arena)
            .map(|id| self.arena.get(id).expect("Child should exist"));

        Ok(children)
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

fn detect_changes_in_nodes(
    n1: Option<&NodeId>,
    n2: Option<&NodeId>,
    arena1: &Arena<Node>,
    arena2: &Arena<Node>,
    path: &mut Path,
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

            if node1.kind != node2.kind
                || node1.role != node2.role
                || (n1.children(arena1).peekable().peek().is_none()
                    && n2.children(arena2).peekable().peek().is_none()
                    && node1.value != node2.value)
            {
                differences.push(Change::Update(path.to_vec(), node2.clone()))
            }

            for (index, (child1, child2)) in
                zip_children_with_none(n1, n2, arena1, arena2).enumerate()
            {
                path.push(index);
                let child_diffs =
                    detect_changes_in_nodes(child1.as_ref(), child2.as_ref(), arena1, arena2, path);
                differences.extend(child_diffs);
                path.pop();
            }
        }
        (Some(_), None) => {
            differences.push(Change::Deletion(path.to_vec()));
        }
        (None, Some(n2)) => {
            let node2 = arena2
                .get(*n2)
                .expect("Node 2 should exist in arena 2")
                .get();

            differences.push(Change::Addition(path.to_vec(), node2.clone()));

            for (index, child) in n2.children(arena2).enumerate() {
                path.push(index);
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
    arena: &mut Arena<Node>,
    level: usize,
    offset: usize,
) -> Vec<NodeChange<'a>> {
    let mut node_changes = Vec::new();

    let relevant_changes: Vec<_> = changes
        .iter()
        .filter(|change| change.path().len() > level && change.path()[level] == offset)
        .cloned()
        .collect();

    if relevant_changes.is_empty() {
        return node_changes;
    }

    let applicable_changes = relevant_changes
        .iter()
        .filter(|change| change.path().len() == level + 1);

    for change in applicable_changes {
        match &change {
            Change::Update(_, new_node) => {
                node_changes.push(NodeChange::Update(node, new_node));
            }
            Change::Deletion(_) => {
                node_changes.push(NodeChange::Deletion(node));
            }
            _ => panic!("Unexpected change kind"),
        }
    }

    let children: Vec<_> = node.children(arena).collect();
    for (index, child) in children.into_iter().enumerate() {
        let child_changes =
            construct_changed_nodes(child, &relevant_changes, arena, level + 1, index);
        node_changes.extend(child_changes);
    }

    let additions = relevant_changes.iter().filter(|change| {
        change.path().len() == level + 2 && matches!(change, Change::Addition { .. })
    });

    for addition in additions {
        if let Change::Addition(_, new_node) = &addition {
            let new_node = arena.new_node(new_node.clone());
            node_changes.push(NodeChange::Addition(new_node, node));

            let new_node_offset = *addition.path().last().expect("Path should not be empty");
            let addition_children = relevant_changes
                .iter()
                .filter(|change| {
                    change.path().len() >= level + 3 && change.path()[level + 1] == new_node_offset
                })
                .cloned()
                .collect();

            node_changes.extend(construct_changed_nodes(
                new_node,
                &addition_children,
                arena,
                level + 1,
                new_node_offset,
            ));
        }
    }

    node_changes
}

fn zip_children_with_none(
    n1: &NodeId,
    n2: &NodeId,
    arena1: &Arena<Node>,
    arena2: &Arena<Node>,
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

fn nodes_equal(n1: &NodeId, n2: &NodeId, arena1: &Arena<Node>, arena2: &Arena<Node>) -> bool {
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
