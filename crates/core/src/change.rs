use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone, Hash)]
pub enum ChangeKind {
    KindUpdate,
    ValueUpdate,
    Addition,
    Deletion,
}

#[derive(Encode, Decode, Debug, Clone, Hash)]
pub struct Change {
    pub path: Vec<u16>,
    pub kind: ChangeKind,
    pub value: Option<String>,
}

impl Change {
    pub fn new(kind: ChangeKind, path: Vec<u16>, value: Option<String>) -> Self {
        Self {
            kind,
            path,
            value,
        }
    }
}