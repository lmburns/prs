pub(crate) mod close;
pub(crate) mod init;
pub(crate) mod open;
pub(crate) mod resize;
pub(crate) mod status;

use clap::{App, AppSettings};

use crate::cmd::arg::{ArgStore, CmdArg};

/// The tomb command definition.
pub(crate) struct CmdTomb;

impl CmdTomb {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("tomb")
            .about("Manage password store Tomb")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(init::CmdInit::build())
            .subcommand(open::CmdOpen::build())
            .subcommand(close::CmdClose::build())
            .subcommand(status::CmdStatus::build())
            .subcommand(resize::CmdResize::build())
            .arg(ArgStore::build())
    }
}
