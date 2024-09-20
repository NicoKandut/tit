use bincode::{self, Decode, Encode};
use std::{
    io::{Read, Write},
    net::TcpStream,
};
use kern;

#[derive(Debug, Encode, Decode, Default)]
pub enum TitClientMessage {
    Hello,
    #[default]
    Error,
    Disconnect,
    DownloadIndex,
    DownloadFile(String),
    UploadFile(kern::Commit),
}

#[derive(Debug, Encode, Decode, Default)]
pub enum TitServerMessage {
    Hello,
    #[default]
    Error,
    Index {
        commits: Vec<String>,
    },
    CommitFile {
        commit: kern::Commit,
    },
}

pub fn write_message(stream: &mut TcpStream, message: impl bincode::enc::Encode) {
    let message_bytes = bincode::encode_to_vec(message, bincode::config::standard())
        .expect("Failed to serialize message");
    let length = message_bytes.len() as u64;
    let length_bytes = length.to_le_bytes();
    stream.write(&length_bytes).expect("Failed to send message");
    stream
        .write(&message_bytes)
        .expect("Failed to send message");
}

pub fn read_message<T: Decode + Default>(stream: &mut TcpStream) -> T {
    let mut length_buffer = [0u8; 8];
    stream
        .read_exact(&mut length_buffer)
        .expect("Failed to read message length");
    let length = u64::from_le_bytes(length_buffer) as usize;
    let mut message_buffer = vec![0u8; length];
    match stream.read_exact(&mut message_buffer) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            return T::default();
        }
    }
    let (message, _) = bincode::decode_from_slice(&message_buffer, bincode::config::standard())
        .expect("Failed to deserialize message");

    message
}
