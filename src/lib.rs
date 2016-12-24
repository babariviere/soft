#[macro_use]
extern crate error_chain;

pub mod client;
pub mod error;
pub mod server;
pub mod types;

pub const DEFAULT_PORT: u16 = 9045;
