use clap::{App, Arg};

/// The tomb resize command definition.
pub(crate) struct CmdResize;

impl CmdResize {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("resize")
            .alias("r")
            .alias("size")
            .alias("grow")
            .about("Resize tomb")
            .arg(
                Arg::new("size")
                    .long("size")
                    .short('S')
                    .value_name("MEGABYTE")
                    .about("Resize tomb to megabytes"),
            )
    }
}
