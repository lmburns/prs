use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, CmdArgFlag};

/// The recipients generate command matcher.
pub(crate) struct GenerateMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> GenerateMatcher<'a> {
    /// Check whether to skip adding key to store.
    pub(crate) fn no_add(&self) -> bool {
        self.matches.is_present("no-add")
    }

    /// Check whether to skip re-encrypting secrets.
    pub(crate) fn no_recrypt(&self) -> bool {
        self.matches.is_present("no-recrypt")
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

impl<'a> Matcher<'a> for GenerateMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("recipients")?
            .subcommand_matches("generate")
            .map(|matches| GenerateMatcher { matches })
    }
}
