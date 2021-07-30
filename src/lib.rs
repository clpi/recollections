//! The purpose of this crate is to provide implementations of
//! data structures & algorithms not in the standard library,
//! and to provide implementations of those already in the library
//! with personal design preferences or other ad-hoc changes.
//! Intended for personal use in projects developed by me,
//! but anyone is free to use should they find it useful.
//!
pub mod data;
pub mod error;
pub mod prelude;
#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "channel")]
pub mod channel;

pub use error::{RecolError, RecolResult};

pub fn init() -> RecolResult<()> {
    Ok(())
}

