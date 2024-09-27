use crate::TIT_DIR;
use std::{env::current_dir, path::PathBuf};

pub fn find_tit_root() -> Option<PathBuf> {
    current_dir()
        .expect("Failed to get current working directory!")
        .ancestors()
        .find(|dir| dir.join(TIT_DIR).exists())
        .map(|dir| dir.to_path_buf())
}
