use super::Matcher;
use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, CmdArgFlag};
use clap::ArgMatches;

/// The one time password list command matcher
#[derive(Debug)]
pub(crate) struct ListMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> ListMatcher<'a> {
    /// OTP account name
    pub(crate) fn list(&self) -> bool {
        self.matches.is_present("list")
    }

    /// Whether to allow a dirty repository for syncing
    pub(crate) fn allow_dirty(&self) -> bool {
        ArgAllowDirty::is_present(self.matches)
    }

    /// Whether to not sync
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
