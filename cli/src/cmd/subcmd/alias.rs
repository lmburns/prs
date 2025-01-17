use clap::{Command, Arg};

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgQuery, ArgStore, CmdArg};

/// The alias command definition.
pub(crate) struct CmdAlias;

impl CmdAlias {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("alias")
            .alias("ln")
            .alias("link")
            .alias("symlink")
            .about("Alias/symlink a secret")
            .long_about("Alias/symlink a secret without duplicating its content")
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
