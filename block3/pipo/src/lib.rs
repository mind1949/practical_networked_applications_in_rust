//! A data store with [Redis serialization protocol (RESP)](https://redis.io/docs/reference/protocol-spec)

mod client;
mod error;
mod server;

pub use client::PipoClient;
pub use error::{PipoError, Result};
pub use server::PipoServer;
