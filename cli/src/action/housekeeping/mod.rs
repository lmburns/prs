pub mod recrypt;

use anyhow::Result;
use clap::ArgMatches;

use crate::cmd::matcher::{HousekeepingMatcher, Matcher};

/// A file housekeeping action.
pub struct Housekeeping<'a> {
    cmd_matches: &'a ArgMatches<'a>,
}

impl<'a> Housekeeping<'a> {
    /// Construct a new housekeeping action.
    pub fn new(cmd_matches: &'a ArgMatches<'a>) -> Self {
        Self { cmd_matches }
    }

    /// Invoke the housekeeping action.
    pub fn invoke(&self) -> Result<()> {
        // Create the command matcher
        let matcher_housekeeping = HousekeepingMatcher::with(self.cmd_matches).unwrap();

        if matcher_housekeeping.recrypt().is_some() {
            return recrypt::Recrypt::new(self.cmd_matches).invoke();
        }

        // Unreachable, clap will print help for missing sub command instead
        unreachable!()
    }
}
