use clap::ArgMatches;

use super::Matcher;

/// The sync remote command matcher.
pub(crate) struct RemoteMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> RemoteMatcher<'a> {
    /// Get the git URL to set.
    pub(crate) fn git_url(&self) -> Option<&str> {
        self.matches.value_of("GIT_URL")
    }
}

impl<'a> Matcher<'a> for RemoteMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("sync")?
            .subcommand_matches("remote")
            .map(|matches| RemoteMatcher { matches })
    }
}
