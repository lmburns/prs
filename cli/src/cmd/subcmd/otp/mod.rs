pub(crate) mod add;
pub(crate) mod list;
pub(crate) mod remove;

use clap::{App, AppSettings};

use crate::cmd::arg::{ArgStore, CmdArg};

/// The recipients command definition.
pub(crate) struct CmdOtp;

impl CmdOtp {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("otp")
            .about("add OTP codes")
            .alias("totp")
            .alias("hotp")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(add::CmdAdd::build())
            .subcommand(list::CmdList::build())
            .subcommand(remove::CmdRemove::build())
            .arg(ArgStore::build())
    }
}
