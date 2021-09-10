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
    otp::{Account, HashFunction, OneTimePassword, OtpFile},
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
    #[allow(clippy::unnecessary_wraps)]
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
            match OneTimePassword::new(&acc.key, acc.totp, &acc.hash_function, acc.counter, None) {
                Ok(otp) =>
                    if acc.totp {
                        println!(
                            "Account: {}\nTOTP: {}",
                            name.green().bold(),
                            otp.generate().red().bold()
                        );
                    } else {
                        println!(
                            "Account: {}\nHOTP: {}",
                            name.green().bold(),
                            otp.generate().red().bold()
                        );
                    },
                Err(err) => eprintln!("{}", err),
            }
        });
        tracing::trace!("list otps: {:#?}", otp_file.list());

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
