use std::fmt::Display;

#[derive(Debug)]
pub enum NetworkError {
    DecodeError,
    EncodeError,
    ReadError,
    WriteError,
    UnexpectedMessage,
}

impl Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NetworkError")
    }
}
