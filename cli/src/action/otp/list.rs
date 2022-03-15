#![allow(unused)]
use anyhow::Result;
use clap::ArgMatches;
use colored::Colorize;
use thiserror::Error;

use crate::{
    cmd::matcher::{
        main::MainMatcher,
        otp::{list::ListMatcher, OtpMatcher},
        Matcher,
    },
    util::{cli, edit, error, sync},
};

use prs_lib::{
    otp::{
        parse_base32, uri_digits, Account, HashFunction, OneTimePassword, OneTimePasswordBuilder,
        OtpFile,
    },
    Plaintext, Secret, Store,
};

#[cfg(all(feature = "tomb", target_os = "linux"))]
use crate::util::tomb;

/// OTP list actions
pub(crate) struct List<'a> {
    cmd_matches: &'a ArgMatches,
}

impl<'a> List<'a> {
    /// Construct a new OTP list action
    pub(crate) fn new(cmd_matches: &'a ArgMatches) -> Self {
        Self { cmd_matches }
    }

    /// Invoke the OTP action
    pub(crate) fn invoke(&self) -> Result<()> {
        let _span = tracing::debug_span!("invoking otp list").entered();

        #[cfg(all(feature = "tomb", target_os = "linux"))]
        let matcher_main = MainMatcher::with(self.cmd_matches).unwrap();
        let matcher_otp = OtpMatcher::with(self.cmd_matches).unwrap();
        let matcher_list = ListMatcher::with(self.cmd_matches).unwrap();

        let store = Store::open(matcher_otp.store()).map_err(Err::Store)?;
        #[cfg(all(feature = "tomb", target_os = "linux"))]
        let mut tomb = store.tomb(
            !matcher_main.verbose(),
            matcher_main.verbose(),
            matcher_main.force(),
        );

        // Prepare tomb
        #[cfg(all(feature = "tomb", target_os = "linux"))]
        tomb::prepare_tomb(&mut tomb, &matcher_main).map_err(Err::Tomb)?;

        let mut otp_file = OtpFile::new(&store)?;

        otp_file.list().iter().for_each(|(name, acc)| {
            match OneTimePasswordBuilder::default()
                .key(parse_base32(&acc.key).unwrap())
                .totp(acc.totp)
                .hash_function(acc.hash_function)
                .counter(acc.counter.unwrap_or_default())
                .period(acc.period)
                .output_len(acc.uri.clone().map_or(6_usize, |ref u| uri_digits(u).unwrap_or_default()))
                .raw_key(acc.key.to_string())
                .build()
            {
                Ok(otp) => {
                    println!(
                        "{} account: {}",
                        if otp.totp { "TOTP" } else { "HOTP" },
                        name.blue().bold()
                    );
                    otp.display_code();
                    println!();
                },
                Err(err) => eprintln!("{}", err),
            }
        });

        // Finalize tomb
        #[cfg(all(feature = "tomb", target_os = "linux"))]
        tomb::finalize_tomb(&mut tomb, &matcher_main, false).map_err(Err::Tomb)?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub(crate) enum Err {
    #[error("failed to access password store")]
    Store(#[source] anyhow::Error),

    #[cfg(all(feature = "tomb", target_os = "linux"))]
    #[error("failed to prepare password store tomb for usage")]
    Tomb(#[source] anyhow::Error),
}
