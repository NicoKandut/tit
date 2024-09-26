use crate::bytes_to_hex;
use crate::change::Change;
use bincode::{Decode, Encode};
use sha3::Digest;
use std::fmt::{Display, Formatter, Write};
use std::hash::Hash;

#[derive(Encode, Decode, Debug, Clone, Hash)]
pub struct Commit {
    pub message: String,
    pub changes: Vec<Change>,
    pub timestamp: u128,
    pub predecessor_id: Option<String>,
}

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

    pub fn encode(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, bincode::config::standard()).unwrap()
    }

    pub fn get_id(&self) -> String {
        let mut hasher = sha3::Sha3_256::default();
        hasher.update(self.encode());
        let hash = hasher.finalize();
        bytes_to_hex(&hash)
    }
}

impl Display for Commit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        f.write_str(&self.get_id()[0..7])?;
        f.write_str("] ")?;
        f.write_str(&self.message)?;
        f.write_str(" (")?;
        f.write_str(&self.timestamp.to_string())?;
        f.write_str(") (")?;
        f.write_str(&self.changes.len().to_string())?;
        f.write_str(" changes)")
    }
}
