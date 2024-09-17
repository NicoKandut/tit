#[derive(Debug, Clone)]
pub struct MutableNode {
    pub kind: String,
    pub value: Option<String>,
}

impl PartialEq for MutableNode {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.value == other.value
    }
}