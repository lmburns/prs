use clap::{Command, Arg};

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgQuery, ArgStore, CmdArg};

/// The duplicate command definition.
pub(crate) struct CmdDuplicate;

impl CmdDuplicate {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("duplicate")
            .alias("dup")
            .about("Duplicate a secret")
            .long_about("Duplicate the contents of a secret to a new file")
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
