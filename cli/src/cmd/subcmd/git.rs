use clap::{App, AppSettings, Arg};

use crate::cmd::arg::{ArgStore, CmdArg};

/// The git command definition.
pub(crate) struct CmdGit;

impl CmdGit {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("git")
            .about("Invoke git command in password store")
            .arg(
                Arg::new("COMMAND")
                    .about("Git command to invoke")
                    .multiple_values(true),
            )
            .arg(ArgStore::build())
            .setting(AppSettings::TrailingVarArg)
    }
}
