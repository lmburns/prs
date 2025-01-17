use clap::{Command, Arg};

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, CmdArg};

/// The recipient remove command definition.
pub(crate) struct CmdRemove;

impl CmdRemove {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("remove")
            .alias("rm")
            .alias("delete")
            .alias("del")
            .about("Remove store recipient")
            .arg(
                Arg::new("recrypt")
                    .long("recrypt")
                    .alias("reencrypt")
                    .help("Re-encrypting all secrets"),
            )
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
    }
}
