// TODO: Add option to not display otp to screen and only copy
pub(crate) mod add;
pub(crate) mod list;
pub(crate) mod remove;
pub(crate) mod view;

use anyhow::Result;
use clap::ArgMatches;

use crate::cmd::matcher::{Matcher, OtpMatcher};

/// An OTP action
pub(crate) struct Otp<'a> {
    cmd_matches: &'a ArgMatches,
}

impl<'a> Otp<'a> {
    /// Construct a new otp action
    pub(crate) fn new(cmd_matches: &'a ArgMatches) -> Self {
        Self { cmd_matches }
    }

    /// Invoke the housekeeping action.
    pub(crate) fn invoke(&self) -> Result<()> {
        // Create the command matcher
        let matcher_otp = OtpMatcher::with(self.cmd_matches).unwrap();

        if matcher_otp.cmd_add().is_some() {
            return add::Add::new(self.cmd_matches).invoke();
        }

        if matcher_otp.cmd_list().is_some() {
            return list::List::new(self.cmd_matches).invoke();
        }

        if matcher_otp.cmd_remove().is_some() {
            return remove::Remove::new(self.cmd_matches).invoke();
        }

        if matcher_otp.cmd_view().is_some() {
            return view::View::new(self.cmd_matches).invoke();
        }

        // Ok(())

        // Unreachable, clap will print help for missing sub command instead
        unreachable!()
    }
}
