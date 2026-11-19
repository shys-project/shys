//! Parsing of protocol version 1 envelopes.
//!
//! The envelope is the byte string stored by the server. Its format is
//! specified in section 6 of `docs/protocol.md`:
//!
//! ```text
//! offset  size    field
//! 0       4       magic       "SHYS"
//! 4       1       version     0x01
//! 5       24      nonce
//! 29      P + 16  ciphertext  including the authentication tag
//! ```
//!
//! Parsing only checks the parts that are not encrypted. It does not
//! decrypt or authenticate anything.

use core::fmt;

use crate::Error;

/// Magic bytes at the start of every envelope.
pub const MAGIC: [u8; 4] = *b"SHYS";

/// Protocol version handled by this module.
pub const VERSION: u8 = 0x01;

/// Length of the nonce, in bytes.
pub const NONCE_LEN: usize = 24;

/// Length of the header (magic, version and nonce), in bytes.
pub const HEADER_LEN: usize = 29;

/// Length of the smallest valid envelope, in bytes.
pub const MIN_ENVELOPE_LEN: usize = 1_069;

/// Length of the largest valid envelope, in bytes.
pub const MAX_ENVELOPE_LEN: usize = 1_081_389;

const _: () = assert!(HEADER_LEN == MAGIC.len() + 1 + NONCE_LEN);

/// A parsed envelope.
///
/// The only way to obtain an `Envelope` is [`Envelope::parse`], so holding
/// one proves that its length, magic and version are valid. It borrows the
/// input and never copies it.
#[derive(Clone, Copy)]
pub struct Envelope<'a> {
    header: &'a [u8; HEADER_LEN],
    nonce: &'a [u8; NONCE_LEN],
    ciphertext: &'a [u8],
}

impl<'a> Envelope<'a> {
    /// Parses an envelope.
    ///
    /// The checks follow the order of section 9, step 4 of the
    /// specification: length, magic and version.
    ///
    /// # Errors
    ///
    /// - [`Error::EnvelopeLength`] if `bytes` is shorter than
    ///   [`MIN_ENVELOPE_LEN`] or longer than [`MAX_ENVELOPE_LEN`].
    /// - [`Error::EnvelopeMagic`] if `bytes` does not start with [`MAGIC`].
    /// - [`Error::UnsupportedVersion`] if the version is not [`VERSION`].
    ///
    /// # Examples
    ///
    /// ```
    /// use shys_core::Error;
    /// use shys_core::envelope::{Envelope, HEADER_LEN, MAGIC, MIN_ENVELOPE_LEN, VERSION};
    ///
    /// let mut bytes = vec![0; MIN_ENVELOPE_LEN];
    /// bytes[..MAGIC.len()].copy_from_slice(&MAGIC);
    /// bytes[MAGIC.len()] = VERSION;
    ///
    /// let envelope = Envelope::parse(&bytes)?;
    /// assert_eq!(envelope.ciphertext().len(), MIN_ENVELOPE_LEN - HEADER_LEN);
    ///
    /// assert_eq!(
    ///     Envelope::parse(b"not an envelope").unwrap_err(),
    ///     Error::EnvelopeLength,
    /// );
    /// # Ok::<(), Error>(())
    /// ```
    pub fn parse(bytes: &'a [u8]) -> Result<Self, Error> {
        if !(MIN_ENVELOPE_LEN..=MAX_ENVELOPE_LEN).contains(&bytes.len()) {
            return Err(Error::EnvelopeLength);
        }

        let (header, ciphertext) = bytes
            .split_first_chunk::<HEADER_LEN>()
            .ok_or(Error::EnvelopeLength)?;
        let [_, _, _, _, version, nonce @ ..] = header;

        if !header.starts_with(&MAGIC) {
            return Err(Error::EnvelopeMagic);
        }
        if *version != VERSION {
            return Err(Error::UnsupportedVersion);
        }

        Ok(Self {
            header,
            nonce,
            ciphertext,
        })
    }

    /// Returns the header, which is the associated data for decryption.
    #[must_use]
    pub fn header(&self) -> &'a [u8; HEADER_LEN] {
        self.header
    }

    /// Returns the nonce.
    #[must_use]
    pub fn nonce(&self) -> &'a [u8; NONCE_LEN] {
        self.nonce
    }

    /// Returns the ciphertext, including the authentication tag.
    #[must_use]
    pub fn ciphertext(&self) -> &'a [u8] {
        self.ciphertext
    }
}

/// Shows only the length of the envelope, never its contents.
impl fmt::Debug for Envelope<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Envelope")
            .field("len", &(HEADER_LEN + self.ciphertext.len()))
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NONCE_BYTE: u8 = 0x11;
    const CIPHERTEXT_BYTE: u8 = 0x22;

    /// Builds a valid envelope of the given length.
    fn envelope(len: usize) -> Vec<u8> {
        let mut bytes = vec![CIPHERTEXT_BYTE; len];
        bytes[..4].copy_from_slice(&MAGIC);
        bytes[4] = VERSION;
        bytes[5..HEADER_LEN].fill(NONCE_BYTE);
        bytes
    }

    #[test]
    fn accepts_the_smallest_envelope() {
        assert!(Envelope::parse(&envelope(MIN_ENVELOPE_LEN)).is_ok());
    }

    #[test]
    fn accepts_the_largest_envelope() {
        assert!(Envelope::parse(&envelope(MAX_ENVELOPE_LEN)).is_ok());
    }

    #[test]
    fn returns_the_parts_of_the_envelope() {
        let bytes = envelope(MIN_ENVELOPE_LEN);
        let parsed = Envelope::parse(&bytes).unwrap();

        assert_eq!(parsed.header().as_slice(), &bytes[..HEADER_LEN]);
        assert_eq!(parsed.nonce(), &[NONCE_BYTE; NONCE_LEN]);
        assert_eq!(parsed.ciphertext(), &bytes[HEADER_LEN..]);
    }

    #[test]
    fn rejects_empty_input() {
        assert_eq!(Envelope::parse(&[]).unwrap_err(), Error::EnvelopeLength);
    }

    #[test]
    fn rejects_an_envelope_one_byte_too_short() {
        let bytes = envelope(MIN_ENVELOPE_LEN - 1);
        assert_eq!(Envelope::parse(&bytes).unwrap_err(), Error::EnvelopeLength);
    }

    #[test]
    fn rejects_an_envelope_one_byte_too_long() {
        let bytes = envelope(MAX_ENVELOPE_LEN + 1);
        assert_eq!(Envelope::parse(&bytes).unwrap_err(), Error::EnvelopeLength);
    }

    #[test]
    fn rejects_every_truncation_without_panicking() {
        let bytes = envelope(MIN_ENVELOPE_LEN);
        for len in 0..MIN_ENVELOPE_LEN {
            assert_eq!(
                Envelope::parse(&bytes[..len]).unwrap_err(),
                Error::EnvelopeLength,
                "length {len}"
            );
        }
    }

    #[test]
    fn rejects_a_wrong_magic() {
        for position in 0..MAGIC.len() {
            let mut bytes = envelope(MIN_ENVELOPE_LEN);
            bytes[position] ^= 0x01;
            assert_eq!(
                Envelope::parse(&bytes).unwrap_err(),
                Error::EnvelopeMagic,
                "byte {position}"
            );
        }
    }

    #[test]
    fn rejects_unknown_versions() {
        for version in [0x00, 0x02, 0xff] {
            let mut bytes = envelope(MIN_ENVELOPE_LEN);
            bytes[4] = version;
            assert_eq!(
                Envelope::parse(&bytes).unwrap_err(),
                Error::UnsupportedVersion,
                "version {version}"
            );
        }
    }

    #[test]
    fn checks_the_length_before_the_magic() {
        let mut bytes = envelope(MIN_ENVELOPE_LEN - 1);
        bytes[0] = 0x00;
        assert_eq!(Envelope::parse(&bytes).unwrap_err(), Error::EnvelopeLength);
    }

    #[test]
    fn checks_the_magic_before_the_version() {
        let mut bytes = envelope(MIN_ENVELOPE_LEN);
        bytes[0] = 0x00;
        bytes[4] = 0x02;
        assert_eq!(Envelope::parse(&bytes).unwrap_err(), Error::EnvelopeMagic);
    }

    #[test]
    fn debug_output_does_not_include_the_contents() {
        let bytes = envelope(MAX_ENVELOPE_LEN);
        let parsed = Envelope::parse(&bytes).unwrap();
        assert_eq!(
            format!("{parsed:?}"),
            format!("Envelope {{ len: {MAX_ENVELOPE_LEN}, .. }}")
        );
    }
}
