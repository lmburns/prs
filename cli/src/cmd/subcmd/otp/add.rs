use clap::{Command, Arg};
use data_encoding::BASE32_NOPAD;
use prs_lib::otp::has_uri;

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgQuery, CmdArg};

/// The add command definition.
pub(crate) struct CmdAdd;

impl CmdAdd {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("add")
            .alias("a")
            .about("Add an otp code")
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
                Arg::new("KEY")
                    .long("key")
                    .short('k')
                    .alias("secret")
                    .takes_value(true)
                    .required_unless_present("uri")
                    .help("Secret key of the OTP")
                    .validator(|p| {
                        BASE32_NOPAD
                            .decode(p.to_uppercase().as_bytes())
                            .map_err(|_| "the key is not a valid base32 encoding")
                            .map(|_| ())
                            .map_err(ToString::to_string)
                    }),
            )
            .arg(
                Arg::new("uri")
                    .long("uri")
                    .short('u')
                    .conflicts_with("KEY")
                    .takes_value(true)
                    .help("URI format of OTP")
                    .validator(|p| {
                        if has_uri(p) {
                            Ok(())
                        } else {
                            Err(String::from("invalid URI format"))
                        }
                    }),
            )
            .arg(
                Arg::new("period")
                    .long("period")
                    .short('p')
                    .value_name("NUM")
                    .takes_value(false)
                    .conflicts_with("hotp")
                    .help("Specify period/interval that account resets")
                    .validator(|p| {
                        p.parse::<u64>()
                            .map_err(|_| "must be a positive number")
                            .map(|_| ())
                            .map_err(ToString::to_string)
                    }),
            )
            .arg(
                Arg::new("totp")
                    .long("totp")
                    .takes_value(false)
                    .conflicts_with("hotp")
                    .help("Time based account (default)"),
            )
            .arg(
                Arg::new("hotp")
                    .long("hotp")
                    .takes_value(false)
                    .help("Counter based account"),
            )
            .arg(
                Arg::new("algorithm")
                    .short('A')
                    .long("algorithm")
                    .takes_value(true)
                    .possible_values(&["SHA1", "SHA256", "SHA384", "SHA512", "SHA512_256"])
                    .value_name("ALGORITHM")
                    .help("Algorithm to use to generate the OTP code"),
            )
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
            .arg(ArgQuery::build())
    }
}
