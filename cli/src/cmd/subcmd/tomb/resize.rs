use clap::{Command, Arg};

/// The tomb resize command definition.
pub(crate) struct CmdResize;

impl CmdResize {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("resize")
            .alias("r")
            .alias("size")
            .alias("grow")
            .about("Resize tomb")
            .arg(
                Arg::new("size")
                    .long("size")
                    .short('S')
                    .value_name("MEGABYTE")
                    .help("Resize tomb to megabytes"),
            )
    }
}
