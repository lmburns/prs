use anyhow::Result;
use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgProperty, ArgQuery, ArgStore, ArgTimeout, CmdArgOption};

/// The show command matcher.
pub(crate) struct ShowMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> ShowMatcher<'a> {
    /// Check whether to just show the first line of the secret.
    pub(crate) fn first_line(&self) -> bool {
        self.matches.is_present("first")
    }

    /// The secret query.
    pub(crate) fn query(&self) -> Option<String> {
        ArgQuery::value(self.matches)
    }

    /// The store.
    pub(crate) fn store(&self) -> String {
        ArgStore::value(self.matches)
    }

    /// Show timeout in seconds.
    pub(crate) fn timeout(&self) -> Option<Result<u64>> {
        ArgTimeout::value(self.matches)
    }

    /// The selected property.
    pub(crate) fn property(&self) -> Option<&str> {
        ArgProperty::value(self.matches)
    }

    /// Check whether to read from copy.
    #[cfg(feature = "clipboard")]
    pub(crate) fn copy(&self) -> bool {
        self.matches.is_present("copy")
    }
}

impl<'a> Matcher<'a> for ShowMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("show")
            .map(|matches| ShowMatcher { matches })
    }
}
