use anyhow::Result;
use secstr::SecVec;
use thiserror::Error;
use zeroize::Zeroize;

/// Newline character(s) on this platform.
#[cfg(not(windows))]
pub const NEWLINE: &str = "\n";
#[cfg(windows)]
pub const NEWLINE: &str = "\r\n";

/// Ciphertext.
///
/// Wraps ciphertext bytes. This type is limited on purpose, to prevent accidentally leaking the
/// ciphertext. Security properties are enforced by `secstr::SecVec`.
pub struct Ciphertext(SecVec<u8>);

impl Ciphertext {
    /// New empty ciphertext.
    pub fn empty() -> Self {
        vec![].into()
    }

    /// Get unsecure reference to inner data.
    ///
    /// # Warning
    ///
    /// Considered unsecure because we cannot guarantee that the referenced data isn't cloned. Use
    /// with care!
    pub(crate) fn unsecure_ref(&self) -> &[u8] {
        self.0.unsecure()
    }
}

impl From<Vec<u8>> for Ciphertext {
    fn from(mut other: Vec<u8>) -> Ciphertext {
        // Explicit zeroing of unsecure buffer required
        let into = Ciphertext(other.to_vec().into());
        other.zeroize();
        into
    }
}

/// Plaintext.
///
/// Wraps plaintext bytes. This type is limited on purpose, to prevent accidentally leaking the
/// plaintext. Security properties are enforced by `secstr::SecVec`.
#[derive(Clone)]
pub struct Plaintext(SecVec<u8>);

impl Plaintext {
    /// New empty plaintext.
    pub fn empty() -> Self {
        vec![].into()
    }

    /// Get unsecure reference to inner data.
    ///
    /// # Warning
    ///
    /// Considered unsecure because we cannot guarantee that the referenced data isn't cloned. Use
    /// with care!
    pub fn unsecure_ref(&self) -> &[u8] {
        self.0.unsecure()
    }

    /// Get the plaintext as UTF8 string.
    ///
    /// # Warning
    ///
    /// Considered unsecure because we cannot guarantee that the referenced data isn't messed with
    /// or isn't cloned.
    pub fn unsecure_to_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.unsecure_ref())
    }

    /// Get the first line of this secret as plaintext.
    ///
    /// Returns empty plaintext if there are no lines.
    pub fn first_line(self) -> Result<Plaintext> {
        Ok(self
            .unsecure_to_str()
            .map_err(Err::FirstLine)?
            .lines()
            .next()
            .map(|l| l.as_bytes().into())
            .unwrap_or_else(|| vec![])
            .into())
    }

    /// Get all lines execpt the first one.
    ///
    /// Returns empty plaintext if there are no lines.
    pub fn except_first_line(self) -> Result<Plaintext> {
        Ok(self
            .unsecure_to_str()
            .map_err(Err::FirstLine)?
            .lines()
            .skip(1)
            .collect::<Vec<&str>>()
            .join(NEWLINE)
            .into_bytes()
            .into())
    }

    /// Append other plaintext.
    ///
    /// Optionally adds platform newline.
    pub fn append(&mut self, other: Plaintext, newline: bool) {
        // TODO: do not use temporary (unsecure) buffer here
        let mut data = self.unsecure_ref().to_vec();
        if newline {
            data.extend_from_slice(NEWLINE.as_bytes());
        }
        data.extend_from_slice(other.unsecure_ref());
        self.0 = data.into();
    }

    /// Check whether this plaintext is empty.
    ///
    /// - Empty if 0 bytes
    /// - Empty if bytes parsed as UTF-8 has trimmed length of 0 characters (ignored on encoding failure)
    pub fn is_empty(&self) -> bool {
        self.unsecure_ref().is_empty()
            || std::str::from_utf8(self.unsecure_ref())
                .map(|s| s.trim().is_empty())
                .unwrap_or(false)
    }
}

impl From<String> for Plaintext {
    fn from(mut other: String) -> Plaintext {
        // Explicit zeroing of unsecure buffer required
        let into = Plaintext(other.as_bytes().into());
        other.zeroize();
        into
    }
}

impl From<Vec<u8>> for Plaintext {
    fn from(mut other: Vec<u8>) -> Plaintext {
        // Explicit zeroing of unsecure buffer required
        let into = Plaintext(other.to_vec().into());
        other.zeroize();
        into
    }
}

impl From<&str> for Plaintext {
    fn from(s: &str) -> Self {
        Self(s.as_bytes().into())
    }
}

#[derive(Debug, Error)]
pub enum Err {
    #[error("failed to select first line of plaintext")]
    FirstLine(#[source] std::str::Utf8Error),
}
