use clap::{Command, Arg};

#[cfg(feature = "clipboard")]
use crate::cmd::arg::ArgTimeout;
use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgStore, CmdArg};

/// The generate command definition.
pub(crate) struct CmdGenerate;

impl CmdGenerate {
    pub(crate) fn build<'a>() -> Command<'a> {
        let cmd = Command::new("generate")
            .alias("gen")
            .alias("g")
            .alias("random")
            .about("Generate a secure secret")
            .arg(
                Arg::new("NAME")
                    .help("Secret name and path")
                    .required_unless_present_any(&["show", "copy"]),
            )
            .arg(
                Arg::new("passphrase")
                    .long("passphrase")
                    .short('P')
                    .help("Generate passhprase instead of random string"),
            )
            .arg(
                Arg::new("length")
                    .value_name("NUM")
                    .long("length")
                    .short('l')
                    .alias("len")
                    .help("Generated password length in characters")
                    .long_help(
                        "Generated password length in characters. Passphrase length in words.",
                    ),
            )
            .arg(
                Arg::new("merge")
                    .long("merge")
                    .short('m')
                    .help("Merge into existing secret, don't create new secret"),
            )
            .arg(
                Arg::new("edit")
                    .long("edit")
                    .short('e')
                    .help("Edit secret after generation"),
            )
            .arg(
                Arg::new("stdin")
                    .long("stdin")
                    .short('S')
                    .alias("from-stdin")
                    .help("Append to generated secret from stdin")
                    .conflicts_with("edit"),
            )
            .arg(
                Arg::new("show")
                    .long("show")
                    .alias("cat")
                    .alias("display")
                    .help("Display secret after generation"),
            )
            .arg(ArgStore::build())
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build());

        #[cfg(feature = "clipboard")]
        let cmd = cmd
            .arg(
                Arg::new("copy")
                    .long("copy")
                    .short('c')
                    .alias("cp")
                    .help("Copy secret to clipboard"),
            )
            .arg(ArgTimeout::build().requires("copy"));

        cmd
    }
}
