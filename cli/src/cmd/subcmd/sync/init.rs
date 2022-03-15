use clap::Command;

/// The sync init command definition.
pub(crate) struct CmdInit;

impl CmdInit {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("init")
            .alias("initialize")
            .about("Initialize sync")
    }
}
