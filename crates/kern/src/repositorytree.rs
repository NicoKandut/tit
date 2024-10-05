use indextree::{Arena, NodeId};
use serde::{Deserialize, Serialize};
use std::{
    fmt, fs,
    path::{Path, PathBuf},
    vec,
};

use crate::{util::BinFile, Change, Node};

#[derive(Clone, Serialize, Deserialize)]
pub enum DirNode {
    Dir { name: String, children: Vec<NodeId> },
    File { name: String, content: String }, // later subtree
}

impl PartialEq for DirNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DirNode::Dir { name: name1, .. }, DirNode::Dir { name: name2, .. }) => name1 == name2,
            (
                DirNode::File {
                    name: name1,
                    content: content1,
                },
                DirNode::File {
                    name: name2,
                    content: content2,
                },
            ) => name1 == name2 && content1 == content2,
            _ => false,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RepositoryTree {
    root_id: NodeId,
    nodes: Arena<DirNode>,
    created_at: u128,
}

impl BinFile for RepositoryTree {}

impl Default for RepositoryTree {
    fn default() -> Self {
        let mut arena = Arena::new();
        let root = arena.new_node(DirNode::Dir {
            name: "..".to_string(),
            children: vec![],
        });
        Self {
            root_id: root,
            nodes: arena,
            created_at: 0,
        }
    }
}

impl RepositoryTree {
    pub fn for_dir(root_dir: &Path) -> Self {
        let mut tree = Self::default();
        tree.created_at = crate::util::get_epoch_millis();
        add_children_to_arena_recursive(&mut tree.nodes, tree.root_id, root_dir);
        tree
    }

    pub fn difference(&self, other: &Self) -> Vec<Change> {
        let mut left = self.to_vec();
        let mut right = other.to_vec();
        let mut difference = vec![];

        for (l_path, l_content) in left.drain(..) {
            if let Some(matching) = right
                .iter()
                .position(|(r_path, r_content)| *r_path == l_path && *r_content == l_content)
            {
                // no change
                right.remove(matching);
            } else if let Some(matching) = right
                .iter()
                .position(|(_, r_content)| *r_content == l_content)
            {
                let r_path = right[matching].0.clone();
                difference.push(Change::Update(
                    vec![],
                    Node {
                        kind: "file".to_string(),
                        value: Some(r_path.file_name().unwrap().to_str().unwrap().to_string()),
                        role: Some("moved".to_string()),
                    },
                ));
                right.remove(matching);
            } else if let Some(matching) = right.iter().position(|(r_path, _)| *r_path == l_path) {
                let r_path = right[matching].0.clone();
                difference.push(Change::Update(
                    vec![],
                    Node {
                        kind: "file".to_string(),
                        value: Some(r_path.file_name().unwrap().to_str().unwrap().to_string()),
                        role: Some("changed".to_string()),
                    },
                ));
                right.remove(matching);
            } else {
                difference.push(Change::Deletion(vec![]));
            }
        }

        // anytihng left over in right is an addition
        for (r_path, _) in right {
            difference.push(Change::Addition(
                vec![],
                Node {
                    kind: "file".to_string(),
                    value: Some(r_path.file_name().unwrap().to_str().unwrap().to_string()),
                    role: Some("new".to_string()),
                },
            ));
        }

        difference
    }

    fn to_vec(&self) -> Vec<(PathBuf, String)> {
        self.nodes
            .iter()
            .map(|node| {
                let id = self.nodes.get_node_id(node).unwrap();
                let path = self.path_of(id);
                let content = match node.get() {
                    DirNode::Dir { .. } => String::new(),
                    DirNode::File { content, .. } => content.clone(),
                };

                (path, content)
            })
            .collect::<Vec<_>>()
    }

    pub fn path_of(&self, node_id: NodeId) -> PathBuf {
        let mut path = vec![];
        let mut cur_node = Some(node_id);

        while let Some(node) = cur_node {
            let node = &self.nodes[node];
            let name = match node.get() {
                DirNode::Dir { name, .. } => name,
                DirNode::File { name, .. } => name,
            };
            path.push(name);

            cur_node = node.parent();
        }

        path.iter().rev().collect::<PathBuf>()
    }
}

impl fmt::Debug for DirNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            DirNode::Dir { name, .. } => name,
            DirNode::File { name, .. } => name,
        };

        write!(f, "{:?}", name)
    }
}

impl fmt::Debug for RepositoryTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = self.root_id.debug_pretty_print(&self.nodes);
        write!(f, "{:?}", printable)
    }
}

fn add_children_to_arena_recursive(arena: &mut Arena<DirNode>, parent: NodeId, path: &Path) {
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    if path.is_dir() {
        let node = arena.new_node(DirNode::Dir {
            name,
            children: vec![],
        });
        parent.append(node, arena);

        let ignored = parse_ignore_file(path);
        path.read_dir()
            .unwrap()
            .filter_map(|e| match e {
                Ok(e) => Some(e),
                Err(_) => None,
            })
            .map(|e| e.path())
            .filter(|e| !ignored.contains(&e.file_name().unwrap().to_str().unwrap().to_string()))
            .for_each(|e| add_children_to_arena_recursive(arena, node, &e));
    } else {
        let content = fs::read_to_string(path).unwrap();
        let node = arena.new_node(DirNode::File { name, content });
        parent.append(node, arena);
    }
}

fn parse_ignore_file(path: &Path) -> Vec<String> {
    let ignore_file = path.join(".titignore");
    if ignore_file.exists() {
        let contents = fs::read_to_string(ignore_file).unwrap();
        contents
            .lines()
            .map(|l| l.to_string())
            .chain([".tit".to_string()])
            .collect::<Vec<_>>()
    } else {
        vec![".tit".to_string()]
    }
}
