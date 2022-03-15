use clap::{Command, Arg};

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgQuery, ArgStore, CmdArg};

/// The edit command definition.
pub(crate) struct CmdEdit;

impl CmdEdit {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("edit")
            .alias("e")
            .about("Edit a secret")
            .arg(ArgQuery::build())
            .arg(
                Arg::new("stdin")
                    .long("stdin")
                    .short('S')
                    .alias("from-stdin")
                    .help("Read secret from stdin, do not open editor"),
            )
            .arg(ArgStore::build())
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
    }
}
