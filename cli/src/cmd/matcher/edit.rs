use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgQuery, ArgStore, CmdArgFlag, CmdArgOption};

/// The edit command matcher.
pub(crate) struct EditMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> EditMatcher<'a> {
    /// The secret query.
    pub(crate) fn query(&self) -> Option<String> {
        ArgQuery::value(self.matches)
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

impl<'a> Matcher<'a> for EditMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("edit")
            .map(|matches| EditMatcher { matches })
    }
}
