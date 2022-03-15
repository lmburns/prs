pub(crate) mod add;
pub(crate) mod export;
pub(crate) mod generate;
pub(crate) mod list;
pub(crate) mod remove;

use clap::Command;

use crate::cmd::arg::{ArgStore, CmdArg};

/// The recipients command definition.
pub(crate) struct CmdRecipients;

impl CmdRecipients {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("recipients")
            .about("Manage store recipients")
            .alias("recipient")
            .alias("recip")
            .alias("rec")
            .alias("keys")
            .alias("kes")
            .subcommand_required(true)
            .subcommand(add::CmdAdd::build())
            .subcommand(export::CmdExport::build())
            .subcommand(generate::CmdGenerate::build())
            .subcommand(list::CmdList::build())
            .subcommand(remove::CmdRemove::build())
            .arg(ArgStore::build())
    }
}
