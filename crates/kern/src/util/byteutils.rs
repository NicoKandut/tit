use bincode::error::{DecodeError, EncodeError};
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

pub fn to_serialized_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, EncodeError> {
    let config = bincode::config::standard();
    let bytes = bincode::serde::encode_to_vec::<_, _>(value, config)?;
    let inflated_bytes = miniz_oxide::deflate::compress_to_vec(&bytes, 10);

    Ok(inflated_bytes)
}

pub fn from_serialized_bytes<T: DeserializeOwned>(
    serialized_bytes: &[u8],
) -> Result<T, DecodeError> {
    let config = bincode::config::standard();
    let bytes = miniz_oxide::inflate::decompress_to_vec(&serialized_bytes)
        .expect("Failed to decompress commit!");
    let (value, _) = bincode::serde::decode_from_slice(&bytes, config)?;
    Ok(value)
}
