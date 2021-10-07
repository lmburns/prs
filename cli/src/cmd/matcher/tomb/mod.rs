pub(crate) mod close;
pub(crate) mod init;
pub(crate) mod open;
pub(crate) mod resize;
pub(crate) mod status;

use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgStore, CmdArgOption};

/// The tomb command matcher.
pub(crate) struct TombMatcher<'a> {
    root: &'a ArgMatches,
    matches: &'a ArgMatches,
}

impl<'a> TombMatcher<'a> {
    /// Get the tomb init sub command, if matched.
    pub(crate) fn cmd_init(&'a self) -> Option<init::InitMatcher> {
        init::InitMatcher::with(self.root)
    }

    /// Get the tomb open sub command, if matched.
    pub(crate) fn cmd_open(&'a self) -> Option<open::OpenMatcher> {
        open::OpenMatcher::with(self.root)
    }

    /// Get the tomb close sub command, if matched.
    pub(crate) fn cmd_close(&'a self) -> Option<close::CloseMatcher> {
        close::CloseMatcher::with(self.root)
    }

    /// Get the tomb status sub command, if matched.
    pub(crate) fn cmd_status(&'a self) -> Option<status::StatusMatcher> {
        status::StatusMatcher::with(self.root)
    }

    /// Get the tomb resize sub command, if matched.
    pub(crate) fn cmd_resize(&'a self) -> Option<resize::ResizeMatcher> {
        resize::ResizeMatcher::with(self.root)
    }

    /// The store.
    pub(crate) fn store(&self) -> String {
        ArgStore::value(self.matches)
    }
}

impl<'a> Matcher<'a> for TombMatcher<'a> {
    fn with(root: &'a ArgMatches) -> Option<Self> {
        root.subcommand_matches("tomb")
            .map(|matches| TombMatcher { root, matches })
    }
}
