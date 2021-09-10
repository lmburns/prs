#[cfg(feature = "clipboard")]
pub(crate) mod clip_revert;
pub(crate) mod completions;

use anyhow::Result;
use clap::ArgMatches;

use crate::cmd::matcher::{InternalMatcher, Matcher};

/// An internal action.
pub(crate) struct Internal<'a> {
    cmd_matches: &'a ArgMatches,
}

impl<'a> Internal<'a> {
    /// Construct a new internal action.
    pub(crate) fn new(cmd_matches: &'a ArgMatches) -> Self {
        Self { cmd_matches }
    }

    /// Invoke the internal action.
    pub(crate) fn invoke(&self) -> Result<()> {
        // Create the command matcher
        let matcher_internal = InternalMatcher::with(self.cmd_matches).unwrap();

        #[cfg(feature = "clipboard")]
        if matcher_internal.clip_revert().is_some() {
            return clip_revert::ClipRevert::new(self.cmd_matches).invoke();
        }

        if matcher_internal.completions().is_some() {
            return completions::Completions::new(self.cmd_matches).invoke();
        }

        // Unreachable, clap will print help for missing sub command instead
        unreachable!()
    }
}
