use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone, Hash)]
pub enum ChangeKind {
    KindUpdate { kind: String, value: Option<String> },
    ValueUpdate(String),
    Addition { kind: String, value: Option<String> },
    Deletion,
}

#[derive(Encode, Decode, Debug, Clone, Hash)]
pub struct Change {
    pub path: Vec<u16>,
    pub kind: ChangeKind
}