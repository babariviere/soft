//! Soft core library
#![deny(missing_docs)]

extern crate app_dirs;
#[macro_use]
extern crate error_chain;

pub mod client;
pub mod error;
pub mod server;
pub mod sync;
pub mod types;

mod common;

/// App information
pub const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo {
    name: "soft",
    author: "notkild",
};

/// Default port used by soft
pub const DEFAULT_PORT: u16 = 9045;
