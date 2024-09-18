use std::fmt::{Display, Formatter};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

use bincode::{Decode, Encode};

use crate::change::Change;

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

    pub fn get_id(&self) -> String {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        let hash = s.finish();
        hash.to_string()
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
        f.write_fmt(format_args!("COMMIT: {}", self.message))
    }
}