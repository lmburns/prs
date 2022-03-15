pub(crate) mod add;
pub(crate) mod list;
pub(crate) mod remove;
pub(crate) mod view;

use clap::Command;

use crate::cmd::arg::{ArgStore, CmdArg};

/// The recipients command definition.
pub(crate) struct CmdOtp;

impl CmdOtp {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("otp")
            .about("add OTP codes")
            .alias("totp")
            .alias("hotp")
            .subcommand_required(true)
            .subcommand(add::CmdAdd::build())
            .subcommand(list::CmdList::build())
            .subcommand(remove::CmdRemove::build())
            .subcommand(view::CmdView::build())
            .arg(ArgStore::build())
    }
}
