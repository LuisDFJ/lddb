use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub enum DBError {
    IO(std::io::Error),
    TryInt(std::num::TryFromIntError),
    Custom(&'static str),
    EOF,
    UnexpectedEOF,
    UnexpectedType,
    BadCRC32,
    Unkown,
}

impl Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBError::IO(err) => write!(f, "I/O Failure: {}", err),
            DBError::TryInt(err) => write!(f, "Try Int Failure: {}", err),
            DBError::Custom(msg) => write!(f, "DB Error: {}", msg),
            DBError::EOF => write!(f, "EOF Reached"),
            DBError::UnexpectedEOF => write!(f, "Unexpected EOF Reached"),
            DBError::UnexpectedType => write!(f, "Unexpected Type"),
            DBError::BadCRC32 => write!(f, "Bad CRC32 checksum"),
            _ => write!(f, "Unkown Failure"),
        }
    }
}

impl Error for DBError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DBError::IO(err) => Some(err),
            DBError::TryInt(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for DBError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::UnexpectedEof => DBError::UnexpectedEOF,
            _ => DBError::IO(value),
        }
    }
}

impl From<std::num::TryFromIntError> for DBError {
    fn from(value: std::num::TryFromIntError) -> Self {
        DBError::TryInt(value)
    }
}
