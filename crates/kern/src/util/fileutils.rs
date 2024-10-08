use super::{from_serialized_bytes, to_serialized_bytes};
use crate::DOT_TIT;
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
        .find(|dir| dir.join(DOT_TIT).exists())
        .map(|dir| dir.to_path_buf())
}

pub trait BinaryFile {}

pub trait BinaryFileRead<P> {
    fn read_from(path: P) -> Self;
}

pub trait BinaryFileWrite<P> {
    fn write_to(&self, path: P);
}

impl<P, T> BinaryFileRead<P> for T
where
    P: AsRef<Path>,
    T: DeserializeOwned + BinaryFile,
{
    fn read_from(path: P) -> Self {
        let mut compressed_bytes = vec![];
        fs::File::open(path)
            .expect("Failed to open binary file!")
            .read_to_end(&mut compressed_bytes)
            .expect("Failed to read binary file!");
        from_serialized_bytes(&compressed_bytes).expect("Failed to deserialize binary file")
    }
}

impl<P, T> BinaryFileWrite<P> for T
where
    P: AsRef<Path>,
    T: Serialize + BinaryFile,
{
    fn write_to(&self, path: P) {
        let compressed_bytes = to_serialized_bytes(self).expect("Failed to serialize!");
        fs::File::create(path)
            .expect("Failed to create file!")
            .write(&compressed_bytes)
            .expect("Failed to write commit!");
    }
}

pub trait TomlFile {}

pub trait TomlFileRead<P> {
    fn read_from(path: P) -> Self;
}

pub trait TomlFileWrite<P> {
    fn write_to(&self, path: P);
}

impl<P, T> TomlFileRead<P> for T
where
    P: AsRef<Path>,
    T: DeserializeOwned + TomlFile,
{
    fn read_from(path: P) -> Self {
        let mut file_content = String::new();
        fs::File::open(path)
            .expect("Failed to open repository state file!")
            .read_to_string(&mut file_content)
            .expect("Failed to read file");
        toml::from_str(&file_content).expect("Failed to parse state file")
    }
}

impl<P, T> TomlFileWrite<P> for T
where
    P: AsRef<Path>,
    T: Serialize + TomlFile,
{
    fn write_to(&self, path: P) {
        let string = toml::to_string_pretty(self).expect("Failed to serialize state");
        fs::File::create(path)
            .expect("Failed to open repository state file!")
            .write_all(string.as_bytes())
            .expect("Failed to read file");
    }
}
