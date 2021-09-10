#![allow(unused)]
use std::{fs, io};

use anyhow::Result;
use clap::ArgMatches;
// use data_encoding::{DecodeError, BASE32_NOPAD};
use std::path::Path;
use thiserror::Error;

use crate::{
    cmd::matcher::{
        main::MainMatcher,
        otp::{remove::RemoveMatcher, OtpMatcher},
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

/// A file completions action.
pub(crate) struct Remove<'a> {
    cmd_matches: &'a ArgMatches,
}

impl<'a> Remove<'a> {
    /// Construct a new OTP add action
    pub(crate) fn new(cmd_matches: &'a ArgMatches) -> Self {
        Self { cmd_matches }
    }

    /// Invoke the OTP action
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn invoke(&self) -> Result<()> {
        let _span = tracing::debug_span!("invoking otp remove").entered();

        let matcher_main = MainMatcher::with(self.cmd_matches).unwrap();
        let matcher_otp = OtpMatcher::with(self.cmd_matches).unwrap();
        let matcher_rm = RemoveMatcher::with(self.cmd_matches).unwrap();

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

        let account_rm = matcher_rm.account();

        if !cli::prompt_yes(
            format!("Remove: {}", account_rm).as_str(),
            Some(true),
            &matcher_main,
        ) {
            error::quit();
        }

        let mut otp_file = OtpFile::new(&store)?;

        if otp_file.get(account_rm).is_some() {
            otp_file.delete(account_rm.to_owned());
            match otp_file.save(&store) {
                Ok(_) => println!("Account successfully deleted"),
                Err(err) => error::print_error(err),
            };
        } else {
            println!("Account does not exist");
        }

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
