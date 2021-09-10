pub(crate) mod recrypt;
pub(crate) mod run;
pub(crate) mod sync_keys;

use anyhow::Result;
use clap::ArgMatches;

use crate::cmd::matcher::{HousekeepingMatcher, Matcher};

/// A file housekeeping action.
pub(crate) struct Housekeeping<'a> {
    cmd_matches: &'a ArgMatches,
}

impl<'a> Housekeeping<'a> {
    /// Construct a new housekeeping action.
    pub(crate) fn new(cmd_matches: &'a ArgMatches) -> Self {
        Self { cmd_matches }
    }

    /// Invoke the housekeeping action.
    pub(crate) fn invoke(&self) -> Result<()> {
        // Create the command matcher
        let matcher_housekeeping = HousekeepingMatcher::with(self.cmd_matches).unwrap();

        if matcher_housekeeping.recrypt().is_some() {
            return recrypt::Recrypt::new(self.cmd_matches).invoke();
        }

        if matcher_housekeeping.run().is_some() {
            return run::Run::new(self.cmd_matches).invoke();
        }

        if matcher_housekeeping.sync_keys().is_some() {
            return sync_keys::SyncKeys::new(self.cmd_matches).invoke();
        }

        // Unreachable, clap will print help for missing sub command instead
        unreachable!()
    }
}
