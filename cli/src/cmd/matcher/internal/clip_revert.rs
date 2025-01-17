use anyhow::Result;
use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::ArgTimeout;

/// The internal clipboard revert command matcher.
pub(crate) struct ClipRevertMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> ClipRevertMatcher<'a> {
    /// Check whether to read previous clipboard contents from stdin as base64 line.
    pub(crate) fn previous_base64_stdin(&self) -> bool {
        self.matches.is_present("previous-base64-stdin")
    }

    /// Clipboard timeout in seconds.
    pub(crate) fn timeout(&self) -> Result<u64> {
        ArgTimeout::value_or_default(self.matches)
    }
}

impl<'a> Matcher<'a> for ClipRevertMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("internal")?
            .subcommand_matches("clip-revert")
            .map(|matches| ClipRevertMatcher { matches })
    }
}
