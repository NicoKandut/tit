use std::{
    fmt::{Display, Formatter},
    io::Error,
};

#[derive(Debug)]
pub struct TitError<'a>(pub &'a str, pub Option<Error>);

impl<'a> Display for TitError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("TitError: {}. Cause: {:?}", self.0, self.1))
    }
}
