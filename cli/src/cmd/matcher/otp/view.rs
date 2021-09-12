use super::Matcher;
use anyhow::Result;
use crate::cmd::arg::{ArgTimeout, CmdArgOption};
use clap::ArgMatches;

/// The one time password list command matcher
#[derive(Debug)]
pub(crate) struct ViewMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> ViewMatcher<'a> {
    /// OTP account name to view
    pub(crate) fn account(&self) -> Option<&str> {
        self.matches.value_of("ACCOUNT")
    }

    /// OTP account name to view
    pub(crate) fn length(&self) -> usize {
        self.matches
            .value_of("length")
            .map_or(6_usize, |v| v.parse::<usize>().unwrap())
    }

    /// Show timeout in seconds
    pub(crate) fn timeout(&self) -> Option<Result<u64>> {
        ArgTimeout::value(self.matches)
    }

    /// Check whether to read from copy.
    #[cfg(feature = "clipboard")]
    pub(crate) fn copy(&self) -> bool {
        self.matches.is_present("copy")
    }
}

impl<'a> Matcher<'a> for ViewMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("otp")?
            .subcommand_matches("view")
            .map(|matches| ViewMatcher { matches })
    }
}
