/// thread pool error
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThreadPool err: {:?}", self)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// alias for std::result::Result<T, Error>
pub type Result<T> = std::result::Result<T, Error>;
