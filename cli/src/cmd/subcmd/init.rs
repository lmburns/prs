use clap::Command;

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgStore, CmdArg};

/// The init command definition.
pub(crate) struct CmdInit;

impl CmdInit {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("init")
            .alias("initialize")
            .about("Initialize new password store")
            .arg(ArgStore::build())
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
    }
}
