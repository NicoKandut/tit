pub fn struct_to_byte_slice<T>(data: &T) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(data as *const T as *const u8, size_of::<T>())
    }
}

pub fn slice_to_byte_slice<T>(data: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * size_of::<T>())
    }
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut acc, byte| {
        acc.push_str(&format!("{:02x}", byte));
        acc
    })
}
