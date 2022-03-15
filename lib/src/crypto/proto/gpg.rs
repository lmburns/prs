//! Crypto GPG protocol.

/// Represents a GPG key.
#[derive(Clone)]
pub struct Key {
    /// Full fingerprint.
    pub fingerprint: String,

    /// Displayable user ID strings.
    pub user_ids: Vec<String>,
}

impl Key {
    /// Key fingerprint.
    #[must_use]
    pub fn fingerprint(&self, short: bool) -> String {
        if short {
            &self.fingerprint[self.fingerprint.len() - 16..]
        } else {
            &self.fingerprint
        }
        .trim()
        .to_uppercase()
    }

    /// Key displayable user data.
    #[must_use]
    pub fn display_user(&self) -> String {
        self.user_ids.join("; ")
    }

    /// Transform into generic key.
    #[must_use]
    pub const fn into_key(self) -> crate::crypto::Key {
        crate::crypto::Key::Gpg(self)
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.fingerprint.trim().to_uppercase() == other.fingerprint.trim().to_uppercase()
    }
}
