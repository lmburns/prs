use clap::{App, Arg};

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgStore, CmdArg};

/// The init command definition.
pub(crate) struct CmdGrep;

impl CmdGrep {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("grep")
            .about("Grep through the password store")
            .arg(
                Arg::new("TERM")
                    .about("Search term")
                    .takes_value(true)
                    .required(true)
            )
            .arg(ArgStore::build())
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
    }
}
