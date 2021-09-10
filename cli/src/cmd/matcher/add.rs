use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgStore, CmdArgFlag, CmdArgOption};

/// The add command matcher.
pub(crate) struct AddMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> AddMatcher<'a> {
    /// Secret destination.
    pub(crate) fn name(&self) -> &str {
        self.matches.value_of("NAME").unwrap()
    }

    /// Check whether to create an empty secret.
    pub(crate) fn empty(&self) -> bool {
        self.matches.is_present("empty")
    }

    /// Check whether to read from stdin.
    pub(crate) fn stdin(&self) -> bool {
        self.matches.is_present("stdin")
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

impl<'a> Matcher<'a> for AddMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("add")
            .map(|matches| AddMatcher { matches })
    }
}
