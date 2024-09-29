
use std::fmt::Display;

use crate::node::Node;
use crate::path::Path;
use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone, Hash)]
pub enum Change {
    Update(Path, Node),
    Addition(Path, Node),
    Deletion(Path),
}

impl Change {
    pub fn path(&self) -> &Path {
        match self {
            Change::Update(path, _) => path,
            Change::Addition(path, _) => path,
            Change::Deletion(path) => path,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, bincode::config::standard()).unwrap()
    }
}

impl Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Change::Update(path, node) => write!(f, "Update: {:?} -> {}", path, node),
            Change::Addition(path, node) => write!(f, "Addition: {:?} -> {}", path, node),
            Change::Deletion(path) => write!(f, "Deletion: {:?}", path),
        }
    }
}