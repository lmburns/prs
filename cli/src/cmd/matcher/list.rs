use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgQuery, ArgStore, CmdArgOption};

/// The list command matcher.
pub(crate) struct ListMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> ListMatcher<'a> {
    /// The secret query.
    pub(crate) fn query(&self) -> Option<String> {
        ArgQuery::value(self.matches)
    }

    /// The store.
    pub(crate) fn store(&self) -> String {
        ArgStore::value(self.matches)
    }

    /// Whether to show as plain list.
    pub(crate) fn list(&self) -> bool {
        self.matches.is_present("list")
    }

    /// Whether to only show aliases.
    pub(crate) fn only_aliases(&self) -> bool {
        self.matches.is_present("aliases")
    }

    /// Whether to only show aliases.
    pub(crate) fn only_non_aliases(&self) -> bool {
        self.matches.is_present("non-aliases")
    }
}

impl<'a> Matcher<'a> for ListMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("list")
            .map(|matches| ListMatcher { matches })
    }
}
