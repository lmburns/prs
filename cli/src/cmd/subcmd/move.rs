use clap::{Command, Arg};

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgQuery, ArgStore, CmdArg};

/// The move command definition.
pub(crate) struct CmdMove;

impl CmdMove {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("move")
            .alias("mov")
            .alias("mv")
            .alias("rename")
            .alias("ren")
            .about("Move a secret")
            .arg(ArgQuery::build().required(true))
            .arg(
                Arg::new("DEST")
                    .help("Secret destination path")
                    .required(true),
            )
            .arg(ArgStore::build())
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
    }
}
