use clap::{Command, Arg};

/// The recipient export command definition.
pub(crate) struct CmdExport;

impl CmdExport {
    pub(crate) fn build<'a>() -> Command<'a> {
        let cmd = Command::new("export")
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
                    .help("Write recipient key to file instead of stdout"),
            );

        #[cfg(feature = "clipboard")]
        let cmd = cmd.arg(
            Arg::new("copy")
                .long("copy")
                .short('c')
                .alias("yank")
                .help("Copy recipient key to clipboard instead of stdout"),
        );

        cmd
    }
}
