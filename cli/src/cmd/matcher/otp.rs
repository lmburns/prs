use super::Matcher;
use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgStore, CmdArgFlag, CmdArgOption};
use clap::ArgMatches;
use std::fmt;

/// The one time password command matcher
pub struct OtpMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> OtpMatcher<'a> {
    /// OTP account name
    pub fn name(&self) -> &str {
        self.matches.value_of("ACCOUNT").unwrap()
    }

    /// Secret key of the OTP
    pub fn key(&self) -> String {
        self.matches.value_of("KEY").unwrap().to_uppercase()
    }

    /// Check whether to use TOTP code
    pub fn totp(&self) -> bool {
        self.matches.is_present("totp")
    }

    /// Check whether to use HOTP code
    pub fn hotp(&self) -> bool {
        self.matches.is_present("hotp")
    }

    /// Check what hashing algorithm to use
    pub fn algorithm(&self) -> HashFunction {
        self.matches
            .value_of("algorithm")
            .map(|val| HashFunction::from_str(val))
            .flatten()
            .unwrap_or(HashFunction::Sha1)
    }

    /// Check what hashing algorithm to as a str
    pub fn algorithm_str(&self) -> &str {
        self.matches.value_of("algorithm").unwrap_or("SHA1")
    }

    /// The store
    pub fn store(&self) -> String {
        ArgStore::value(self.matches)
    }

    /// Whether to allow a dirty repository for syncing
    pub fn allow_dirty(&self) -> bool {
        ArgAllowDirty::is_present(self.matches)
    }

    /// Whether to not sync
    pub fn no_sync(&self) -> bool {
        ArgNoSync::is_present(self.matches)
    }
}

impl<'a> Matcher<'a> for OtpMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("otp")
            .map(|matches| OtpMatcher { matches })
    }
}

/// Available hashing algorithms
#[derive(Copy, Clone)]
pub enum HashFunction {
    Sha1,
    Sha256,
    Sha384,
    Sha512,
}

impl HashFunction {
    /// List all supported hashing functions
    pub fn variants() -> &'static [HashFunction] {
        &[
            HashFunction::Sha1,
            HashFunction::Sha256,
            HashFunction::Sha384,
            HashFunction::Sha512,
        ]
    }

    /// Convert `str` to variant
    pub fn from_str(hash: &str) -> Option<HashFunction> {
        match hash.trim().to_ascii_lowercase().as_str() {
            "sha1" | "1" => Some(HashFunction::Sha1),
            "sha256" | "256" => Some(HashFunction::Sha256),
            "sha384" | "384" => Some(HashFunction::Sha384),
            "sha512" | "512" => Some(HashFunction::Sha512),
            _ => None,
        }
    }

    /// Get hash function name
    pub fn name(self) -> &'static str {
        match self {
            HashFunction::Sha1 => "sha1",
            HashFunction::Sha256 => "sha256",
            HashFunction::Sha384 => "sha384",
            HashFunction::Sha512 => "sha512",
        }
    }
}

impl fmt::Display for HashFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
