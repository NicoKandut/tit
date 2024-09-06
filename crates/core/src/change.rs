use bincode::{Decode, Encode};

use crate::change_kind::ChangeKind;

#[derive(Encode, Decode, Debug, Clone, Hash)]
pub struct Change {
    kind: ChangeKind,
    address: String,
    node: String,
}

impl Change {
    pub fn new(kind: ChangeKind, address: String, node: String) -> Self {
        Self {
            kind,
            address,
            node,
        }
    }
}