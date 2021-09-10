use clap::{App, Arg};
use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, CmdArg};

/// The list command definition.
pub(crate) struct CmdRemove;

impl CmdRemove {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("remove")
            .alias("r")
            .alias("rm")
            .about("remove otp account")
            .arg(
                Arg::new("ACCOUNT")
                    .alias("file")
                    .alias("service")
                    .takes_value(true)
                    .required(true)
                    .about("Name of the account/file to remove"),
            )
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
    }
}
