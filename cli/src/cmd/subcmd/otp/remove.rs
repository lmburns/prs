use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, CmdArg};
use clap::{Command, Arg};

/// The list command definition.
pub(crate) struct CmdRemove;

impl CmdRemove {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("remove")
            .alias("r")
            .alias("rm")
            .about("remove otp account")
            .arg(
                Arg::new("ACCOUNT")
                    .long("account")
                    .short('a')
                    .alias("file")
                    .alias("service")
                    .takes_value(true)
                    .required(false)
                    .help("Name of the account/file to remove"),
            )
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
    }
}
