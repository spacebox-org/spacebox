extern crate ignore;
extern crate notify;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

/// Handles the configuration file format.
pub mod config;

/// Defines the overarching error type for Spacebox
pub mod error;
pub use error::{Error, Result};

/// The entry point to the daemon.  Runs an event handling loop.
pub fn main() {
    println!("Hello, world!");
}
