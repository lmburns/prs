pub mod crypto;
pub(crate) mod git;
pub mod otp;
pub mod store;
pub mod sync;
#[cfg(all(feature = "tomb", target_os = "linux"))]
pub mod systemd_bin;
#[cfg(all(feature = "tomb", target_os = "linux"))]
pub mod tomb;
#[cfg(all(feature = "tomb", target_os = "linux"))]
pub(crate) mod tomb_bin;
pub mod types;
pub mod util;
mod vendor;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

// Re-exports
pub use crypto::{recipients::Recipients, Key};
pub use store::{Secret, Store};
pub use types::{Ciphertext, Plaintext};

use crate::crypto::{Config, Proto};
use once_cell::sync::Lazy;
use std::path::PathBuf;

/// Default password store directory.
#[cfg(not(windows))]
pub const STORE_DEFAULT_ROOT: &str = "~/.password-store";
#[cfg(windows)]
pub const STORE_DEFAULT_ROOT: &str = "~\\.password-store";

/// File location to store files with OTP
pub const OTP_DEFUALT_FILE_LOCATION: Lazy<PathBuf> = Lazy::new(|| {
    vendor::shellexpand::full(STORE_DEFAULT_ROOT)
        .map(|val| PathBuf::from(val.to_string()))
        .unwrap_or_else(|_| {
            dirs_next::home_dir()
                .map(|d| d.join(".password-store"))
                .expect("Invalid password store directory")
        }).join(".otp-codes.json")
});
#[cfg(windows)]
pub const OTP_DEFUALT_FILE: Lazy<String> =
    Lazy::new(|| format!("{}/.otp-codes.json", STORE_DEFAULT_ROOT));

    /// File name where OTP codes are stored
pub const OTP_DEFUALT_FILE: &str = ".otp-codes.json";

/// Default proto config.
// TODO: remove when multiple protocols are supported.
const CONFIG: Config = Config {
    proto:   Proto::Gpg,
    gpg_tty: false,
};
