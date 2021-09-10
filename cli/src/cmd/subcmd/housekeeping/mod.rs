pub(crate) mod recrypt;
pub(crate) mod run;
pub(crate) mod sync_keys;

use clap::{App, AppSettings};

use crate::cmd::arg::{ArgStore, CmdArg};

/// The housekeeping command definition.
pub(crate) struct CmdHousekeeping;

impl CmdHousekeeping {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("housekeeping")
            .about("Housekeeping utilities")
            .alias("housekeep")
            .alias("hk")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(recrypt::CmdRecrypt::build())
            .subcommand(run::CmdRun::build())
            .subcommand(sync_keys::CmdSyncKeys::build())
            .arg(ArgStore::build())
    }
}
