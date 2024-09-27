use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_epoch_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get time!")
        .as_millis()
}