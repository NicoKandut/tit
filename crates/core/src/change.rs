use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone, Hash)]
pub enum Change {
    KindUpdate { path: Vec<u16>, update: String },
    ValueUpdate { path: Vec<u16>, update: String },
    Addition { path: Vec<u16>, kind: String, value: Option<String> },
    Deletion { path: Vec<u16> },
}