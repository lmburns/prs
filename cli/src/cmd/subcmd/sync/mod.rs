pub(crate) mod init;
pub(crate) mod remote;

use clap::App;

use crate::cmd::arg::{ArgAllowDirty, ArgStore, CmdArg};

/// The sync command definition.
pub(crate) struct CmdSync;

impl CmdSync {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("sync")
            .alias("s")
            .about("Sync password store")
            .subcommand(init::CmdInit::build())
            .subcommand(remote::CmdRemote::build())
            .arg(ArgStore::build())
            .arg(ArgAllowDirty::build())
    }
}
