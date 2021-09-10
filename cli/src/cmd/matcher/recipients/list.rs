use clap::ArgMatches;

use super::Matcher;

/// The recipients list command matcher.
pub(crate) struct ListMatcher<'a> {
    _matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> ListMatcher<'a> {}

impl<'a> Matcher<'a> for ListMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("recipients")?
            .subcommand_matches("list")
            .map(|matches| ListMatcher { _matches: matches })
    }
}
