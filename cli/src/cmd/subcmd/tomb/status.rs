use clap::{App, Arg};

/// The tomb status command definition.
pub(crate) struct CmdStatus;

impl CmdStatus {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("status").about("Query tomb status").arg(
            Arg::new("open")
                .long("open")
                .alias("o")
                .about("Open tomb is it is closed"),
        )
    }
}
