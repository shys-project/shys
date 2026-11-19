//! The error type of `shys-core`.

use core::fmt;

/// Errors returned by `shys-core`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The envelope is shorter or longer than the protocol allows.
    EnvelopeLength,
    /// The data does not start with the Shys magic bytes.
    EnvelopeMagic,
    /// The data uses a protocol version this library does not support.
    UnsupportedVersion,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::EnvelopeLength => "envelope length is out of range",
            Self::EnvelopeMagic => "data is not a Shys envelope",
            Self::UnsupportedVersion => "unsupported protocol version",
        };
        f.write_str(message)
    }
}

impl core::error::Error for Error {}
