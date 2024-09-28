use std::io;

pub enum TemplateLoadingError {
    NotFoundError,
    IoError(io::Error),
}