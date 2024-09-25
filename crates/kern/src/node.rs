use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone, Hash)]
pub struct Node {
    pub kind: String,
    pub value: Option<String>,
    pub role: Option<String>
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.value == other.value && self.role == other.role
    }
}