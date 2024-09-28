use bincode::{Decode, Encode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::TIT_DIR;
use std::{
    env::current_dir,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use super::{from_compressed_bytes, from_serialized_bytes, to_compressed_bytes, to_serialized_bytes};

pub fn find_tit_root() -> Option<PathBuf> {
    current_dir()
        .expect("Failed to get current working directory!")
        .ancestors()
        .find(|dir| dir.join(TIT_DIR).exists())
        .map(|dir| dir.to_path_buf())
}

pub trait FileRead<P: AsRef<Path>> {
    fn read_from(path: P) -> Self;
}

pub trait FileWrite<P: AsRef<Path>> {
    fn write_to(&self, path: P);
}

impl<P: AsRef<Path>, T: Decode> FileRead<P> for T {
    fn read_from(path: P) -> Self {
        let mut compressed_bytes = vec![];
        fs::File::open(path)
            .expect("Failed to open commit file!")
            .read_to_end(&mut compressed_bytes)
            .expect("Failed to read commit file!");
        from_compressed_bytes(&compressed_bytes)
    }
}

impl<P: AsRef<Path>, T: Encode> FileWrite<P> for T {
    fn write_to(&self, path: P) {
        let compressed_bytes = to_compressed_bytes(self);
        fs::File::create(path)
            .expect("Failed to create file!")
            .write(&compressed_bytes)
            .expect("Failed to write commit!");
    }
}