use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgStore, CmdArgOption};

/// The show command matcher.
pub(crate) struct GrepMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> GrepMatcher<'a> {
    pub(crate) fn search(&self) -> &str {
        self.matches.value_of("TERM").unwrap()
    }

    /// The store.
    pub(crate) fn store(&self) -> String {
        ArgStore::value(self.matches)
    }
}

impl<'a> Matcher<'a> for GrepMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("grep")
            .map(|matches| GrepMatcher { matches })
    }
}
