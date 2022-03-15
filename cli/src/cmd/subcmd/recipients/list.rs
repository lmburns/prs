use clap::Command;

/// The recipient list command definition.
pub(crate) struct CmdList;

impl CmdList {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("list")
            .alias("ls")
            .alias("l")
            .about("List store recipients")
    }
}
