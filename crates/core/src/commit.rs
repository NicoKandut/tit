use std::hash::{DefaultHasher, Hash, Hasher};

use bincode::{Decode, Encode};

use crate::change::Change;

#[derive(Encode, Decode, Debug, Clone, Hash)]
pub struct Commit {
    message: String,
    changes: Vec<Change>,
}

impl Commit {
    pub fn new(message: String, changes: Vec<Change>) -> Self {
        Self { message, changes }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn get_id(&self) -> String {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        let hash = s.finish();
        hash.to_string()
    }
}
