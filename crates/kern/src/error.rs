use std::{
    fmt::{Display, Formatter},
    io::Error,
    path::PathBuf,
};

#[derive(Debug)]
pub struct TitError<'a>(pub &'a str, pub Option<Error>);

impl<'a> Display for TitError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("TitError: {}. Cause: {:?}", self.0, self.1))
    }
}

#[derive(Debug)]
pub enum InitError {
    AlreadyInitialized,
    NotInitialized,
    DirectoryCreateError(PathBuf),
    DirectoryNotFound,
}

impl Display for InitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InitError::AlreadyInitialized => f.write_str("Repository already initialized"),
            InitError::NotInitialized => f.write_str("Repository not initialized"),
            InitError::DirectoryCreateError(path) => {
                f.write_fmt(format_args!("Failed to create directory: {:?}", path))
            }
            InitError::DirectoryNotFound => f.write_str("Directory not found"),
        }
    }
}
