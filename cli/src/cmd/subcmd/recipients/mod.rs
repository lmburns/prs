pub(crate) mod add;
pub(crate) mod export;
pub(crate) mod generate;
pub(crate) mod list;
pub(crate) mod remove;

use clap::{App, AppSettings};

use crate::cmd::arg::{ArgStore, CmdArg};

/// The recipients command definition.
pub(crate) struct CmdRecipients;

impl CmdRecipients {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("recipients")
            .about("Manage store recipients")
            .alias("recipient")
            .alias("recip")
            .alias("rec")
            .alias("keys")
            .alias("kes")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(add::CmdAdd::build())
            .subcommand(export::CmdExport::build())
            .subcommand(generate::CmdGenerate::build())
            .subcommand(list::CmdList::build())
            .subcommand(remove::CmdRemove::build())
            .arg(ArgStore::build())
    }
}
