use super::Matcher;
use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, CmdArgFlag};
use clap::ArgMatches;

/// The one time password list command matcher
#[derive(Debug)]
pub(crate) struct ListMatcher<'a> {
    matches: &'a ArgMatches,
}

// TODO: Cleanup or implement
#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> ListMatcher<'a> {
    /// Whether to allow a dirty repository for syncing
    #[allow(dead_code)]
    pub(crate) fn allow_dirty(&self) -> bool {
        ArgAllowDirty::is_present(self.matches)
    }

    /// Whether to not sync
    #[allow(dead_code)]
    pub(crate) fn no_sync(&self) -> bool {
        ArgNoSync::is_present(self.matches)
    }
}

impl<'a> Matcher<'a> for ListMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("otp")?
            .subcommand_matches("list")
            .map(|matches| ListMatcher { matches })
    }
}
