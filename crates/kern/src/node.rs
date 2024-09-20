use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone, Hash)]
pub struct Node {
    pub kind: String,
    pub value: Option<String>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.value == other.value
    }
}