use anyhow::Result;
use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgProperty, ArgQuery, ArgStore, ArgTimeout, CmdArgOption};

/// The copy command matcher.
pub(crate) struct CopyMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> CopyMatcher<'a> {
    /// Check whether to copy all of the secret.
    pub(crate) fn all(&self) -> bool {
        self.matches.is_present("all")
    }

    /// The secret query.
    pub(crate) fn query(&self) -> Option<String> {
        ArgQuery::value(self.matches)
    }

    /// Clipboard timeout in seconds.
    pub(crate) fn timeout(&self) -> Result<u64> {
        ArgTimeout::value_or_default(self.matches)
    }

    /// The store.
    pub(crate) fn store(&self) -> String {
        ArgStore::value(self.matches)
    }

    /// The selected property.
    pub(crate) fn property(&self) -> Option<&str> {
        ArgProperty::value(self.matches)
    }
}

impl<'a> Matcher<'a> for CopyMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("copy")
            .map(|matches| CopyMatcher { matches })
    }
}
