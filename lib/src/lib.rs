#![feature(derive_default_enum)]
pub mod crypto;
pub(crate) mod git;
#[cfg(feature = "otp")]
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
use std::env;

#[cfg(feature = "otp")]
use std::path::PathBuf;

/// Default password store directory.
#[cfg(not(windows))]
pub const STORE_DEFAULT_ROOT: &str = "~/.password-store";
#[cfg(windows)]
pub const STORE_DEFAULT_ROOT: &str = "~\\.password-store";

/// File location to store files with OTP
#[cfg(feature = "otp")]
pub const OTP_DEFUALT_FILE_LOCATION: Lazy<PathBuf> = Lazy::new(|| {
    vendor::shellexpand::full(STORE_DEFAULT_ROOT)
        .map(|val| PathBuf::from(val.to_string()))
        .unwrap_or_else(|_| {
            dirs_next::home_dir()
                .map(|d| d.join(".password-store"))
                .expect("Invalid password store directory")
        })
        .join(".otp-codes.json")
});

#[cfg(all(feature = "otp", target_os = "windows"))]
pub const OTP_DEFUALT_FILE: Lazy<String> =
    Lazy::new(|| format!("{}/.otp-codes.json", STORE_DEFAULT_ROOT));

/// File name where OTP codes are stored
#[cfg(all(not(windows), feature = "otp"))]
pub const OTP_DEFUALT_FILE: &str = ".otp-codes.json";

/// TODO: How to for windows?
/// `Fortress` UMASK
pub static STORE_UMASK: Lazy<u32> = Lazy::new(|| {
    u32::from_str_radix(
        &env::var("PASSWORD_STORE_UMASK").unwrap_or_else(|_| "077".to_owned()),
        8,
    )
    .expect("umask is not a valid octal")
});

/// Default proto config.
// TODO: remove when multiple protocols are supported.
const CONFIG: Config = Config {
    proto:   Proto::Gpg,
    gpg_tty: false,
};
