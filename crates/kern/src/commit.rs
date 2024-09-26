use std::fmt::{Display, Formatter, Write};
use std::hash::Hash;
use std::time::{SystemTime, UNIX_EPOCH};

use bincode::{Decode, Encode};
use sha3::Digest;

use crate::change::Change;
use crate::{bytes_to_hex, struct_to_byte_slice};

#[derive(Encode, Decode, Debug, Clone, Hash)]
pub struct Commit {
    message: String,
    changes: Vec<Change>,
    timestamp: u128,
}

impl Commit {
    pub fn new(message: String, changes: Vec<Change>, timestamp: u128) -> Self {
        Self {
            message,
            changes,
            timestamp,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, bincode::config::standard()).unwrap()
    }

    pub fn get_id(&self) -> String {
        let mut hasher = sha3::Sha3_256::default();
        hasher.update(self.as_bytes());
        let hash = hasher.finalize();
        bytes_to_hex(&hash)
    }
}

pub fn get_epoch_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get time!")
        .as_millis()
}

impl Display for Commit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        f.write_str(&self.get_id()[0..7])?;
        f.write_str("]: ")?;
        f.write_str(&self.message)?;
        f.write_str(" (")?;
        f.write_str(&self.timestamp.to_string())?;
        f.write_str(") (")?;
        f.write_str(&self.changes.len().to_string())?;
        f.write_str(" changes)")
    }
}
