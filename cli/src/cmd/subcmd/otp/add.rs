use clap::{App, Arg};
use data_encoding::BASE32_NOPAD;

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, CmdArg, ArgQuery};

/// The add command definition.
pub(crate) struct CmdAdd;

impl CmdAdd {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("add")
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
                    .about("Name of the account/file"),
            )
            .arg(
                Arg::new("KEY")
                    .long("key")
                    .short('k')
                    .alias("secret")
                    .takes_value(true)
                    .required(true)
                    .about("Secret key of the OTP")
                    .validator(
                        |arg| match BASE32_NOPAD.decode(arg.to_uppercase().as_bytes()) {
                            Ok(_) => Ok(()),
                            Err(_) => Err(String::from("the key is not a valid base32 encoding")),
                        },
                    ),
            )
            .arg(
                Arg::new("totp")
                    .long("totp")
                    .takes_value(false)
                    .conflicts_with("hotp")
                    .about("Time based account (default)"),
            )
            .arg(
                Arg::new("hotp")
                    .long("hotp")
                    .takes_value(false)
                    .about("Counter based account"),
            )
            .arg(
                Arg::new("algorithm")
                    .short('A')
                    .long("algorithm")
                    .takes_value(true)
                    .possible_values(&["SHA1", "SHA256", "SHA384", "SHA512", "SHA512_256"])
                    .value_name("ALGORITHM")
                    .about("Algorithm to use to generate the OTP code"),
            )
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
            .arg(ArgQuery::build())
    }
}
