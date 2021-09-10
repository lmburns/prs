use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgQuery, CmdArgFlag, CmdArgOption};

/// The housekeeping recrypt command matcher.
pub(crate) struct RecryptMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> RecryptMatcher<'a> {
    /// The secret query.
    pub(crate) fn query(&self) -> Option<String> {
        ArgQuery::value(self.matches)
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

impl<'a> Matcher<'a> for RecryptMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("housekeeping")?
            .subcommand_matches("recrypt")
            .map(|matches| RecryptMatcher { matches })
    }
}
