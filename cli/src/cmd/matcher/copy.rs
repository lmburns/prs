use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgQuery, CmdArgOption};

/// The copy command matcher.
pub struct CopyMatcher<'a> {
    matches: &'a ArgMatches<'a>,
}

impl<'a: 'b, 'b> CopyMatcher<'a> {
    /// Check whether to copy all of the secret.
    pub fn all(&self) -> bool {
        self.matches.is_present("all")
    }

    /// The secret query.
    pub fn query(&self) -> Option<String> {
        ArgQuery::value(self.matches)
    }
}

impl<'a> Matcher<'a> for CopyMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("copy")
            .map(|matches| CopyMatcher { matches })
    }
}