use std::fmt;
use std::io;
use std::str::Utf8Error;
use aes_gcm::aead::Error as AesError;

#[derive(Debug)]
pub enum TaskError {
    Io(io::Error),
    Utf8(Utf8Error),
    Aes(AesError),
    Hex(hex::FromHexError),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::Io(err) => write!(f, "IO error: {}", err),
            TaskError::Utf8(err) => write!(f, "UTF-8 error: {}", err),
            TaskError::Aes(err) => write!(f, "AES error: {:?}", err),
            TaskError::Hex(err) => write!(f, "Hex error: {}", err),
        }
    }
}

impl std::error::Error for TaskError {}

impl From<io::Error> for TaskError {
    fn from(err: io::Error) -> TaskError {
        TaskError::Io(err)
    }
}

impl From<Utf8Error> for TaskError {
    fn from(err: Utf8Error) -> TaskError {
        TaskError::Utf8(err)
    }
}

impl From<AesError> for TaskError {
    fn from(err: AesError) -> TaskError {
        TaskError::Aes(err)
    }
}

impl From<hex::FromHexError> for TaskError {
    fn from(err: hex::FromHexError) -> TaskError {
        TaskError::Hex(err)
    }
}
