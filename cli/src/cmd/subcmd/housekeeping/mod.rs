pub(crate) mod recrypt;
pub(crate) mod run;
pub(crate) mod sync_keys;

use clap::Command;

use crate::cmd::arg::{ArgStore, CmdArg};

/// The housekeeping command definition.
pub(crate) struct CmdHousekeeping;

impl CmdHousekeeping {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("housekeeping")
            .about("Housekeeping utilities")
            .alias("housekeep")
            .alias("hk")
            .subcommand_required(true)
            .subcommand(recrypt::CmdRecrypt::build())
            .subcommand(run::CmdRun::build())
            .subcommand(sync_keys::CmdSyncKeys::build())
            .arg(ArgStore::build())
    }
}
