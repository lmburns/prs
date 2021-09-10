use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, CmdArgFlag};

/// The recipients remove command matcher.
pub(crate) struct RemoveMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> RemoveMatcher<'a> {
    /// Check whether to re-encrypt secrets.
    pub(crate) fn recrypt(&self) -> bool {
        self.matches.is_present("recrypt")
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

impl<'a> Matcher<'a> for RemoveMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("recipients")?
            .subcommand_matches("remove")
            .map(|matches| RemoveMatcher { matches })
    }
}
