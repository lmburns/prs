use clap::{App, Arg};
use data_encoding::BASE32_NOPAD;

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, ArgStore, CmdArg};

/// The add command definition.
pub struct CmdOtp;

impl CmdOtp {
    pub fn build<'a>() -> App<'a> {
        App::new("otp")
            .about("Add an otp code")
            .arg(
                Arg::new("ACCOUNT")
                    .required(true)
                    .about("Name of the account"),
            )
            .arg(
                Arg::new("KEY")
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
                    .conflicts_with("hotp")
                    .about("Time based account (default)"),
            )
            .arg(Arg::new("hotp").long("hotp").about("Counter based account"))
            .arg(
                Arg::new("algorithm")
                    .short('a')
                    .long("algorithm")
                    .takes_value(true)
                    .possible_values(&["SHA1", "SHA256", "SHA384", "SHA512", "SHA512_256"])
                    .value_name("ALGORITHM")
                    .about("Algorithm to use to generate the OTP code"),
            )
            .arg(ArgStore::build())
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
    }
}
