use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub enum StorageError {
    IO(std::io::Error),
    TryInt(std::num::TryFromIntError),
    Custom(&'static str),
    EOF,
    UnexpectedEOF,
    BadCRC32,
    Unkown,
}

impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::IO(err) => write!(f, "I/O Failure: {}", err),
            StorageError::TryInt(err) => write!(f, "Try Int Failure: {}", err),
            StorageError::Custom(msg) => write!(f, "Storage Error: {}", msg),
            StorageError::EOF => write!(f, "EOF Reached"),
            StorageError::UnexpectedEOF => write!(f, "Unexpected EOF Reached"),
            StorageError::BadCRC32 => write!(f, "Bad CRC32 checksum"),
            _ => write!(f, "Unkown Failure"),
        }
    }
}

impl Error for StorageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            StorageError::IO(err) => Some(err),
            StorageError::TryInt(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for StorageError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::UnexpectedEof => StorageError::UnexpectedEOF,
            _ => StorageError::IO(value),
        }
    }
}

impl From<std::num::TryFromIntError> for StorageError {
    fn from(value: std::num::TryFromIntError) -> Self {
        StorageError::TryInt(value)
    }
}
