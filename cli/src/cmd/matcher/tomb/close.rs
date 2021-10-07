use clap::ArgMatches;

use super::Matcher;

/// The tomb close command matcher.
pub(crate) struct CloseMatcher<'a> {
    matches: &'a ArgMatches,
}

impl CloseMatcher<'_> {
    /// Whether to try to close.
    pub(crate) fn do_try(&self) -> bool {
        self.matches.is_present("try")
    }
}

impl<'a> Matcher<'a> for CloseMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("tomb")?
            .subcommand_matches("close")
            .map(|matches| CloseMatcher { matches })
    }
}
