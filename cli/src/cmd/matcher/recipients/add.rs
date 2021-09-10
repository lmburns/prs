use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, CmdArgFlag};

/// The recipients add command matcher.
#[derive(Debug)]
pub(crate) struct AddMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> AddMatcher<'a> {
    /// Check whether to skip re-encrypting secrets.
    pub(crate) fn no_recrypt(&self) -> bool {
        self.matches.is_present("no-recrypt")
    }

    /// Check whether to add a secret key.
    pub(crate) fn secret(&self) -> bool {
        self.matches.is_present("secret")
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

impl<'a> Matcher<'a> for AddMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("recipients")?
            .subcommand_matches("add")
            .map(|matches| AddMatcher { matches })
    }
}
