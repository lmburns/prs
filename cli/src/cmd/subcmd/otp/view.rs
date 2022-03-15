use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, CmdArg, ArgTimeout};
use clap::{Command, Arg};

/// The list command definition.
pub(crate) struct CmdView;

impl CmdView {
    pub(crate) fn build<'a>() -> Command<'a> {
        let cmd = Command::new("view")
            .alias("v")
            .about("view an otp account")
            .arg(
                Arg::new("ACCOUNT")
                    .long("account")
                    .short('a')
                    .takes_value(true)
                    .required(false)
                    .alias("file")
                    .alias("service")
                    .help("Name of the account/file"),
            )
            .arg(
                Arg::new("length")
                    .short('l')
                    .long("length")
                    .takes_value(true)
                    .value_name("NUMBER")
                    .help("Length of the OTP code")
                    .validator(|n| {
                        n.parse::<usize>()
                            .map_err(|_| "value must be a number")
                            .map(|_| ())
                            .map_err(ToString::to_string)
                    }),
            )
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
            .arg(ArgTimeout::build().help("Timeout after which to clear output"));

        #[cfg(feature = "clipboard")]
        let cmd = cmd.arg(
            Arg::new("copy")
                .long("copy")
                .short('c')
                .alias("cp")
                .help("Copy otp to clipboard"),
        );

        cmd
    }
}
