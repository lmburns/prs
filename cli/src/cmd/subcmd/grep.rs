use clap::{Command, Arg};

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgStore, CmdArg};

/// The init command definition.
pub(crate) struct CmdGrep;

impl CmdGrep {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("grep")
            .about("Grep through the password store")
            .arg(
                Arg::new("TERM")
                    .help("Search term")
                    .takes_value(true)
                    .required(true)
            )
            .arg(ArgStore::build())
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
    }
}
