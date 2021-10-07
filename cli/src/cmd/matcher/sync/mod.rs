pub(crate) mod init;
pub(crate) mod remote;

use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgAllowDirty, ArgStore, CmdArgFlag, CmdArgOption};

/// The sync command matcher.
pub(crate) struct SyncMatcher<'a> {
    root: &'a ArgMatches,
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> SyncMatcher<'a> {
    /// Get the sync init sub command, if matched.
    pub(crate) fn cmd_init(&'a self) -> Option<init::InitMatcher> {
        init::InitMatcher::with(self.root)
    }

    /// Get the sync remote sub command, if matched.
    pub(crate) fn cmd_remote(&'a self) -> Option<remote::RemoteMatcher> {
        remote::RemoteMatcher::with(self.root)
    }

    /// The store.
    pub(crate) fn store(&self) -> String {
        ArgStore::value(self.matches)
    }

    /// Whether to allow a dirty repository for syncing.
    pub(crate) fn allow_dirty(&self) -> bool {
        ArgAllowDirty::is_present(self.matches)
    }
}

impl<'a> Matcher<'a> for SyncMatcher<'a> {
    fn with(root: &'a ArgMatches) -> Option<Self> {
        root.subcommand_matches("sync")
            .map(|matches| SyncMatcher { root, matches })
    }
}
