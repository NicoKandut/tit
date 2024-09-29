use std::fmt::Display;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Encode, Decode, Debug, Clone, Hash)]
pub struct Node {
    pub kind: String,
    pub value: Option<String>,
    pub role: Option<String>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.value == other.value && self.role == other.role
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.value.as_ref(), self.role.as_ref()) {
            (Some(value), Some(role)) => write!(f, "{} {:?} {:?}", self.kind, value, role),
            (Some(value), None) => write!(f, "{} {:?}", self.kind, value),
            (None, Some(role)) => write!(f, "{} {:?}", self.kind, role),
            (None, None) => write!(f, "{}", self.kind),
        }
    }
}
