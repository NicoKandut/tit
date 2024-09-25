use bincode::{Decode, Encode};
use crate::node::Node;
use crate::path::Path;

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
}