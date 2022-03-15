//! Crypto backend using GnuPG for GPG.

pub mod context;
pub mod raw;
mod raw_cmd;

use std::path::PathBuf;

/// GPG config.
pub struct Config {
    /// GPG binary.
    bin: PathBuf,

    /// Use TTY for GPG password input, rather than GUI pinentry.
    pub gpg_tty: bool,
}

impl Config {
    /// Construct with given binary.
    ///
    /// - `config`: path to `gpg` binary
    #[must_use]
    pub const fn from(bin: PathBuf) -> Self {
        Self {
            bin,
            gpg_tty: false,
        }
    }
}
