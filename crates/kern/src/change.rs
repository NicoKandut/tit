use std::default;

use crate::node::Node;
use crate::path::Path;
use bincode::{Decode, Encode};
use sha3::Digest;

#[derive(Encode, Decode, Debug, Clone, Hash)]
pub enum Change {
    KindUpdate(Path, Node),
    ValueUpdate(Path, Node),
    Addition(Path, Node),
    Deletion(Path),
}

impl Change {
    pub fn path(&self) -> &Path {
        match self {
            Change::KindUpdate(path, _) => path,
            Change::ValueUpdate(path, _) => path,
            Change::Addition(path, _) => path,
            Change::Deletion(path) => path,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, bincode::config::standard()).unwrap()
    }
}
