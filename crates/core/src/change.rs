use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone, Copy, Hash)]
pub enum ChangeKind {
    ADDITION = 0,
    DELETION = 1,
    UPDATE = 2,
}

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