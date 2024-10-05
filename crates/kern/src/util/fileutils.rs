use super::{from_serialized_bytes, to_serialized_bytes};
use crate::TIT_DIR;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    env::current_dir,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub fn find_tit_root() -> Option<PathBuf> {
    current_dir()
        .expect("Failed to get current working directory!")
        .ancestors()
        .find(|dir| dir.join(TIT_DIR).exists())
        .map(|dir| dir.to_path_buf())
}

pub trait FileRead<P> {
    fn read_from(path: P) -> Self;
}

pub trait FileWrite<P> {
    fn write_to(&self, path: P);
}

pub trait BinFile {}

impl<P, T> FileRead<P> for T
where
    P: AsRef<Path>,
    T: DeserializeOwned + BinFile,
{
    fn read_from(path: P) -> Self {
        let mut compressed_bytes = vec![];
        fs::File::open(path)
            .expect("Failed to open commit file!")
            .read_to_end(&mut compressed_bytes)
            .expect("Failed to read commit file!");
        from_serialized_bytes(&compressed_bytes)
    }
}

impl<P, T> FileWrite<P> for T
where
    P: AsRef<Path>,
    T: Serialize + BinFile,
{
    fn write_to(&self, path: P) {
        let compressed_bytes = to_serialized_bytes(self);
        fs::File::create(path)
            .expect("Failed to create file!")
            .write(&compressed_bytes)
            .expect("Failed to write commit!");
    }
}
