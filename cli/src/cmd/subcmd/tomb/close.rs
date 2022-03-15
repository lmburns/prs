use clap::{Command, Arg};

/// The tomb close command definition.
pub(crate) struct CmdClose;

impl CmdClose {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("close")
            .alias("c")
            .alias("stop")
            .about("Close tomb")
            .arg(
                Arg::new("try")
                    .long("try")
                    .help("Try to close, don't fail if already closed"),
            )
    }
}
