use clap::App;

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgStore, CmdArg};

/// The init command definition.
pub(crate) struct CmdInit;

impl CmdInit {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("init")
            .alias("initialize")
            .about("Initialize new password store")
            .arg(ArgStore::build())
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
    }
}
