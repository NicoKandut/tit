use bincode::{self, Decode, Encode};
use kern;
use std::{
    collections::BTreeMap,
    io::{Read, Write},
    net::TcpStream,
};

mod client;
mod error;

pub use client::*;
pub use error::*;

#[derive(Debug, Encode, Decode, Default)]
pub enum TitClientMessage {
    Hello,
    #[default]
    Error,
    Disconnect,
    CreateRepository {
        name: String,
    },
    UseRepository {
        name: String,
    },
    DownloadIndex,
    DownloadFile {
        id: String,
    },
    UploadChanges {
        changes: kern::Commit,
    },
    OfferContent {
        commits: Vec<String>,
        branches: BTreeMap<String, String>,
    },
}

#[derive(Debug, Encode, Decode, Default)]
pub enum TitServerMessage {
    Hello,
    Ok,
    #[default]
    Error,
    Index {
        commits: Vec<String>,
        branches: BTreeMap<String, String>,
    },
    CommitFile {
        commit: kern::Commit,
    },
    RepositoryCreated,
    RequestUpload {
        commits: Vec<String>,
    },
}

pub fn write_message(
    stream: &mut TcpStream,
    message: impl bincode::enc::Encode,
) -> Result<(), NetworkError> {
    let message_bytes = bincode::encode_to_vec(message, bincode::config::standard())
        .map_err(|_| NetworkError::EncodeError)?;
    let length = message_bytes.len() as u64;
    let length_bytes = length.to_le_bytes();

    stream
        .write(&length_bytes)
        .map_err(|_| NetworkError::WriteError)?;
    stream
        .write(&message_bytes)
        .map_err(|_| NetworkError::WriteError)?;

    Ok(())
}

pub fn read_message<T: Decode + Default>(stream: &mut TcpStream) -> Result<T, NetworkError> {
    let mut length_buffer = [0u8; 8];
    stream
        .read_exact(&mut length_buffer)
        .map_err(|_| NetworkError::ReadError)?;
    let length = u64::from_le_bytes(length_buffer) as usize;
    let mut message_buffer = vec![0u8; length];
    stream
        .read_exact(&mut message_buffer)
        .map_err(|_| NetworkError::ReadError)?;
    let (message, _) = bincode::decode_from_slice(&message_buffer, bincode::config::standard())
        .map_err(|_| NetworkError::DecodeError)?;

    Ok(message)
}
