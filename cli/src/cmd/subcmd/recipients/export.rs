use clap::{App, Arg};

/// The recipient export command definition.
pub(crate) struct CmdExport;

impl CmdExport {
    pub(crate) fn build<'a>() -> App<'a> {
        let cmd = App::new("export")
            .alias("exp")
            .alias("ex")
            .about("Export recipient key")
            .arg(
                Arg::new("output-file")
                    .long("output-file")
                    .short('o')
                    .alias("output")
                    .alias("file")
                    .value_name("PATH")
                    .about("Write recipient key to file instead of stdout"),
            );

        #[cfg(feature = "clipboard")]
        let cmd = cmd.arg(
            Arg::new("copy")
                .long("copy")
                .short('c')
                .alias("yank")
                .about("Copy recipient key to clipboard instead of stdout"),
        );

        cmd
    }
}
