// use std::error::Error as StdError;
use std::fmt;

pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    IO,
    FS,
    DATABASE,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::IO => write!(f, "io fail"),
            ErrorKind::FS => write!(f, "fs fail"),
            ErrorKind::DATABASE => write!(f, "database"),
        }
    }
}
