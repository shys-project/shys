//! Cryptographic core and envelope format for Shys.
//!
//! This crate implements the Shys protocol, version 1, as specified in
//! `docs/protocol.md`.

#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::indexing_slicing
    )
)]
#![warn(missing_docs)]

pub mod envelope;
mod error;

pub use error::Error;
