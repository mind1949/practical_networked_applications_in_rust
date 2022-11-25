use std::convert::From;
use std::error::Error;
use std::io;

/// Error type for kvs.
#[derive(Debug)]
pub enum KvsError {
    /// IO error.
    Io(io::Error),
    /// Serialization or deserialization error.
    Serde(serde_json::Error),
    /// Removing non-existent key error.
    KeyNotFound,
    /// Unexpected command type error.
    /// It indicated a corrupted log or a program bug.
    UnexpectedCommandType,
    /// Error with a string message
    StringError(String),
    /// sled error
    Sled(sled::Error),
    /// Key or value is invalid UTF-8 sequence
    Utf8(std::string::FromUtf8Error),
}

impl std::fmt::Display for KvsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "kvstore err: {:?}", self)
    }
}

impl Error for KvsError {}

impl From<io::Error> for KvsError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}

impl From<sled::Error> for KvsError {
    fn from(err: sled::Error) -> KvsError {
        KvsError::Sled(err)
    }
}

impl From<std::string::FromUtf8Error> for KvsError {
    fn from(err: std::string::FromUtf8Error) -> KvsError {
        KvsError::Utf8(err)
    }
}

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, KvsError>;
