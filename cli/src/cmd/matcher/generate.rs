#[cfg(feature = "clipboard")]
use anyhow::Result;
use clap::ArgMatches;

use super::Matcher;
#[cfg(feature = "clipboard")]
use crate::cmd::arg::ArgTimeout;
use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgStore, CmdArgFlag, CmdArgOption};

/// Default password length in characters.
const PASSWORD_LENGTH: u16 = 24;

/// Default passphrase length in words.
const PASSPHRASE_LENGTH: u16 = 5;

/// The generate command matcher.
pub(crate) struct GenerateMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> GenerateMatcher<'a> {
    /// Secret name.
    pub(crate) fn name(&self) -> Option<&str> {
        self.matches.value_of("NAME")
    }

    /// Check whether to generate a passphrase.
    pub(crate) fn passphrase(&self) -> bool {
        self.matches.is_present("passphrase")
    }

    /// What length to use.
    pub(crate) fn length(&self) -> u16 {
        self.matches
            .value_of("length")
            .map(|l| l.parse().expect("invalid length"))
            .unwrap_or_else(|| {
                if self.passphrase() {
                    PASSPHRASE_LENGTH
                } else {
                    PASSWORD_LENGTH
                }
            })
    }

    /// Check whether to merge the secret.
    pub(crate) fn merge(&self) -> bool {
        self.matches.is_present("merge")
    }

    /// Check whether to edit the secret.
    pub(crate) fn edit(&self) -> bool {
        self.matches.is_present("edit")
    }

    /// Check whether to read from stdin.
    pub(crate) fn stdin(&self) -> bool {
        self.matches.is_present("stdin")
    }

    /// Check whether to read from copy.
    #[cfg(feature = "clipboard")]
    pub(crate) fn copy(&self) -> bool {
        self.matches.is_present("copy")
    }

    /// Clipboard timeout in seconds.
    #[cfg(feature = "clipboard")]
    pub(crate) fn timeout(&self) -> Result<u64> {
        ArgTimeout::value_or_default(self.matches)
    }

    /// Check whether to read from show.
    pub(crate) fn show(&self) -> bool {
        self.matches.is_present("show")
    }

    /// The store.
    pub(crate) fn store(&self) -> String {
        ArgStore::value(self.matches)
    }

    /// Whether to allow a dirty repository for syncing.
    pub(crate) fn allow_dirty(&self) -> bool {
        ArgAllowDirty::is_present(self.matches)
    }

    /// Whether to not sync.
    pub(crate) fn no_sync(&self) -> bool {
        ArgNoSync::is_present(self.matches)
    }
}

impl<'a> Matcher<'a> for GenerateMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("generate")
            .map(|matches| GenerateMatcher { matches })
    }
}
