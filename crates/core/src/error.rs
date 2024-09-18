use std::fmt::{Display, Formatter, Pointer};

pub struct RepositoryError<'a>(pub &'a str);

impl <'a> Display for RepositoryError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}