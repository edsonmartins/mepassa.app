//! FFI Module for UniFFI bindings
//!
//! This module exposes the MePassa core library to Kotlin and Swift
//! using UniFFI automatic bindings generation.

mod client;
mod types;

pub use client::MePassaClient;
pub use types::*;
