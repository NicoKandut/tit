use crate::change::Change;
use crate::util::{bytes_to_hex, to_serialized_bytes, BinaryFile};
use serde::{Deserialize, Serialize};
use sha3::Digest;
use std::fmt::{Display, Formatter, Write};
use std::hash::Hash;

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct Commit {
    pub message: String,
    pub changes: Vec<Change>,
    pub timestamp: u128,
    pub predecessor_id: Option<String>,
}

impl BinaryFile for Commit {}

impl Commit {
    pub fn new(
        message: String,
        changes: Vec<Change>,
        timestamp: u128,
        predecessor_id: Option<String>,
    ) -> Self {
        Self {
            message,
            changes,
            timestamp,
            predecessor_id,
        }
    }

    pub fn get_id(&self) -> String {
        let mut hasher = sha3::Sha3_256::default();
        let bytes = to_serialized_bytes(&self).expect("Failed to serialize commit");
        hasher.update(&bytes);
        let hash = hasher.finalize();
        let id = bytes_to_hex(&hash);

        id
    }

    pub fn shorten_id(id: &str) -> &str {
        &id[0..7]
    }
}

impl Display for Commit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        f.write_str(&Self::shorten_id(&self.get_id()))?;
        f.write_str("] ")?;
        f.write_str(&self.message)?;
        f.write_str(" (")?;
        f.write_str(&self.timestamp.to_string())?;
        f.write_str(") (")?;
        f.write_str(&self.changes.len().to_string())?;
        f.write_str(" changes)")
    }
}
