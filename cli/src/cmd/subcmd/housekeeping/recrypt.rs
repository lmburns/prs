use clap::{App, Arg};

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgQuery, CmdArg};

/// The housekeeping recrypt command definition.
pub(crate) struct CmdRecrypt;

impl CmdRecrypt {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("recrypt")
            .alias("reencrypt")
            .about("Re-encrypt secrets")
            .arg(
                Arg::new("all")
                    .long("all")
                    .short('a')
                    .about("Re-encrypt all secrets")
                    .conflicts_with("QUERY"),
            )
            .arg(ArgQuery::build().required_unless_present("all"))
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
    }
}
