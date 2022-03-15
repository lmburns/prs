use clap::{Command, Arg};

use crate::cmd::arg::{ArgStore, CmdArg};

/// The git command definition.
pub(crate) struct CmdGit;

impl CmdGit {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("git")
            .about("Invoke git command in password store")
            .arg(
                Arg::new("COMMAND")
                    .help("Git command to invoke")
                    .multiple_values(true),
            )
            .arg(ArgStore::build())
            .trailing_var_arg(true)
    }
}
