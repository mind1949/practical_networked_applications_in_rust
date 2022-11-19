/// Error type for pipo.
#[derive(Debug)]
pub enum PipoError {
    Io(std::io::Error),
}

impl std::fmt::Display for PipoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "kvstore err: {:?}", self)
    }
}

impl From<std::io::Error> for PipoError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub type Result<T> = std::result::Result<T, PipoError>;
