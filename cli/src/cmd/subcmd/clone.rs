use clap::{Command, Arg};

use crate::cmd::arg::{ArgStore, CmdArg};

/// The clone command definition.
pub(crate) struct CmdClone;

impl CmdClone {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("clone")
            .about("Clone existing password store")
            .arg(
                Arg::new("GIT_URL")
                    .help("Remote git URL to clone from")
                    .required(true),
            )
            .arg(ArgStore::build())
    }
}
