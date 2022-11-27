mod error;
mod pool;

#[cfg(test)]
mod test;

pub use error::{Error, Result};
pub use pool::ThreadPool;
