#![deny(missing_docs)]
//! A simple key/value store.

pub use client::KvsClient;
pub use engine::KvsEngine;
pub use error::{KvsError, Result};
pub use server::KvsServer;

mod client;
mod common;
mod engine;
mod engines;
mod error;
mod server;

pub use engines::{KvStore, SledKvsEngine};
