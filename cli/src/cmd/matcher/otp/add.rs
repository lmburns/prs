use super::Matcher;
use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgQuery, CmdArgFlag, CmdArgOption};
use clap::ArgMatches;
use prs_lib::otp::HashFunction;

/// The one time password add command matcher
#[derive(Debug)]
pub(crate) struct AddMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> AddMatcher<'a> {
    /// The secret query.
    pub(crate) fn query(&self) -> Option<String> {
        ArgQuery::value(self.matches)
    }

    // TODO: Implement an option to not link an OTP to an existing `Secret`?
    /// OTP account name
    #[allow(dead_code)]
    pub(crate) fn name(&self) -> Option<&str> {
        self.matches.value_of("ACCOUNT")
    }

    /// Secret key of the OTP (will panic if -k isn't used)
    pub(crate) fn key(&self) -> String {
        self.matches.value_of("KEY").unwrap().to_uppercase()
    }

    /// URI formatted secret
    pub(crate) fn uri(&self) -> Option<&str> {
        self.matches.value_of("uri")
    }

    /// Has URI formatted secret
    #[allow(dead_code)]
    pub(crate) fn has_uri(&self) -> bool {
        self.matches.is_present("uri")
    }

    /// Get period
    pub(crate) fn period(&self) -> u64 {
        self.matches
            .value_of("period")
            .map_or(30_u64, |p| p.parse::<u64>().unwrap())
    }

    /// Check whether to use TOTP code
    pub(crate) fn totp(&self) -> bool {
        self.matches.is_present("totp")
    }

    /// Check whether to use HOTP code
    pub(crate) fn hotp(&self) -> bool {
        self.matches.is_present("hotp")
    }

    /// Check what hashing algorithm to use
    pub(crate) fn algorithm(&self) -> HashFunction {
        self.matches
            .value_of("algorithm")
            .map_or(HashFunction::Sha1, HashFunction::from_str)
    }

    /// Check what hashing algorithm to as a str
    #[allow(dead_code)]
    pub(crate) fn algorithm_str(&self) -> &str {
        self.matches.value_of("algorithm").unwrap_or("Sha1")
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

impl<'a> Matcher<'a> for AddMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("otp")?
            .subcommand_matches("add")
            .map(|matches| AddMatcher { matches })
    }
}
