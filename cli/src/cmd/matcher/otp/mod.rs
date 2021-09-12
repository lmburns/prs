pub(crate) mod add;
pub(crate) mod list;
pub(crate) mod remove;
pub(crate) mod view;

use clap::ArgMatches;
use crate::cmd::arg::{ArgStore, CmdArgOption};
use super::Matcher;

/// The recipients matcher.
pub(crate) struct OtpMatcher<'a> {
    root: &'a ArgMatches,
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> OtpMatcher<'a> {
    /// Get the options from adding an OTP code
    pub(crate) fn cmd_add(&'a self) -> Option<add::AddMatcher> {
        add::AddMatcher::with(self.root)
    }

    /// Get the options for listing OTP codes
    pub(crate) fn cmd_list(&'a self) -> Option<list::ListMatcher> {
        list::ListMatcher::with(self.root)
    }

    /// Get the options for removing OTP codes
    pub(crate) fn cmd_remove(&'a self) -> Option<remove::RemoveMatcher> {
        remove::RemoveMatcher::with(self.root)
    }

    /// Get the options for viewing OTP codes
    pub(crate) fn cmd_view(&'a self) -> Option<view::ViewMatcher> {
        view::ViewMatcher::with(self.root)
    }

    /// The store
    pub(crate) fn store(&self) -> String {
        ArgStore::value(self.matches)
    }
}

impl<'a> Matcher<'a> for OtpMatcher<'a> {
    fn with(root: &'a ArgMatches) -> Option<Self> {
        root.subcommand_matches("otp")
            .map(|matches| OtpMatcher { root, matches })
    }
}
