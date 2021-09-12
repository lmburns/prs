use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, CmdArg, ArgTimeout};
use clap::{App, Arg};

/// The list command definition.
pub(crate) struct CmdView;

impl CmdView {
    pub(crate) fn build<'a>() -> App<'a> {
        let cmd = App::new("view")
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
                    .about("Name of the account/file"),
            )
            .arg(
                Arg::new("length")
                    .short('l')
                    .long("length")
                    .takes_value(true)
                    .value_name("NUMBER")
                    .about("Length of the OTP code")
                    .validator(|n| {
                        n.parse::<usize>()
                            .map_err(|_| "value must be a number")
                            .map(|_| ())
                            .map_err(ToString::to_string)
                    }),
            )
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
            .arg(ArgTimeout::build().about("Timeout after which to clear output"));

        #[cfg(feature = "clipboard")]
        let cmd = cmd.arg(
            Arg::new("copy")
                .long("copy")
                .short('c')
                .alias("cp")
                .about("Copy otp to clipboard"),
        );

        cmd
    }
}
