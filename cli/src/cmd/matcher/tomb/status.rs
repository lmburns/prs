use clap::ArgMatches;

use super::Matcher;

/// The tomb status command matcher.
pub(crate) struct StatusMatcher<'a> {
    matches: &'a ArgMatches,
}

impl StatusMatcher<'_> {
    /// Check whether to open the tomb.
    pub(crate) fn open(&self) -> bool {
        self.matches.is_present("open")
    }
}

impl<'a> Matcher<'a> for StatusMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("tomb")?
            .subcommand_matches("status")
            .map(|matches| StatusMatcher { matches })
    }
}
