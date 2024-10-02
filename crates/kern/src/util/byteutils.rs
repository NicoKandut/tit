use bincode::{Decode, Encode};
use serde::{de::DeserializeOwned, Serialize};

pub fn struct_to_byte_slice<T>(data: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(data as *const T as *const u8, size_of::<T>()) }
}

pub fn slice_to_byte_slice<T>(data: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * size_of::<T>()) }
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut acc, byte| {
        acc.push_str(&format!("{:02x}", byte));
        acc
    })
}

pub fn to_compressed_bytes<T: Encode>(value: &T) -> Vec<u8> {
    let config = bincode::config::standard();
    let bytes = bincode::encode_to_vec::<_, _>(value, config).expect("Failed to encode commit!");
    miniz_oxide::deflate::compress_to_vec(&bytes, 10)
}

pub fn from_compressed_bytes<T: Decode>(compressed_bytes: &[u8]) -> T {
    let config = bincode::config::standard();
    let bytes = miniz_oxide::inflate::decompress_to_vec(&compressed_bytes)
        .expect("Failed to decompress commit!");
    let (value, _) = bincode::decode_from_slice(&bytes, config).expect("Failed to decode commit");
    value
}

pub fn to_serialized_bytes<T: Serialize>(value: &T) -> Vec<u8> {
    let config = bincode::config::standard();
    let bytes =
        bincode::serde::encode_to_vec::<_, _>(value, config).expect("Failed to encode commit!");
    miniz_oxide::deflate::compress_to_vec(&bytes, 10)
}

pub fn from_serialized_bytes<T: DeserializeOwned>(serialized_bytes: &[u8]) -> T {
    let config = bincode::config::standard();
    let bytes = miniz_oxide::inflate::decompress_to_vec(&serialized_bytes)
        .expect("Failed to decompress commit!");
    let (value, _) =
        bincode::serde::decode_from_slice(&bytes, config).expect("Failed to decode commit");
    value
}
