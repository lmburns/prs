use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgStore, CmdArgOption};

/// The clone command matcher.
pub(crate) struct CloneMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> CloneMatcher<'a> {
    /// The git URL to clone from.
    pub(crate) fn git_url(&self) -> &str {
        self.matches.value_of("GIT_URL").unwrap()
    }

    /// The store.
    pub(crate) fn store(&self) -> String {
        ArgStore::value(self.matches)
    }
}

impl<'a> Matcher<'a> for CloneMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("clone")
            .map(|matches| CloneMatcher { matches })
    }
}
