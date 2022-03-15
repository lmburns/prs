use clap::{Command, Arg};

/// The tomb status command definition.
pub(crate) struct CmdStatus;

impl CmdStatus {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("status").about("Query tomb status").arg(
            Arg::new("open")
                .long("open")
                .alias("o")
                .help("Open tomb is it is closed"),
        )
    }
}
