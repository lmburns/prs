#![allow(unused)]
use clap::ArgMatches;
use colored::Colorize;
use std::{fs, io, path::Path, thread, time::Duration};
use anyhow::Result;
use thiserror::Error;

use crate::{
    cmd::matcher::{
        main::MainMatcher,
        otp::{view::ViewMatcher, OtpMatcher},
        Matcher,
    },
    util::{cli, clipboard, edit, error, select, sync},
};

use prs_lib::{
    otp::{Account, HashFunction, OneTimePassword, OtpFile},
    Plaintext, Secret, Store,
};

#[cfg(all(feature = "tomb", target_os = "linux"))]
use crate::util::tomb;

/// A file completions action.
pub(crate) struct View<'a> {
    cmd_matches: &'a ArgMatches,
}

impl<'a> View<'a> {
    /// Construct a new OTP add action
    pub(crate) fn new(cmd_matches: &'a ArgMatches) -> Self {
        Self { cmd_matches }
    }

    /// Invoke the OTP action
    pub(crate) fn invoke(&self) -> Result<()> {
        let _span = tracing::debug_span!("invoking otp view").entered();

        let matcher_main = MainMatcher::with(self.cmd_matches).unwrap();
        let matcher_otp = OtpMatcher::with(self.cmd_matches).unwrap();
        let matcher_view = ViewMatcher::with(self.cmd_matches).unwrap();

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
        let account = if let Some(acc) = matcher_view.account() {
            acc.to_string()
        } else {
            let sec = select::select_otp(&otp_file).ok_or(Err::NoneSelected)?;
            sec.name.clone()
        };

        match otp_file.get(&account) {
            Some(acc) => {
                match OneTimePassword::new(
                    &acc.key,
                    acc.totp,
                    &acc.hash_function,
                    acc.counter,
                    Some(matcher_view.length()),
                ) {
                    Ok(otp) => {
                        println!("{}", otp.generate().green().bold());

                        // Copy to clipboard
                        #[cfg(feature = "clipboard")]
                        if matcher_view.copy() {
                            clipboard::copy_timeout(
                                otp.generate().as_bytes(),
                                matcher_view
                                    .timeout()
                                    .unwrap_or(Ok(crate::CLIPBOARD_TIMEOUT))?,
                                !matcher_main.quiet(),
                            )?;
                        }

                        if let Some(timeout) = matcher_view.timeout() {
                            let timeout = timeout?;
                            #[allow(clippy::cast_possible_truncation)]
                            let mut lines = 1_u16 + 1;

                            if matcher_main.verbose() {
                                lines += 2;
                                eprintln!();
                                eprint!("Clearing output in {} seconds...", timeout);
                            }

                            thread::sleep(Duration::from_secs(timeout));
                            eprint!("{}", ansi_escapes::EraseLines(lines));
                        }
                    },
                    Err(err) => error::print_error(err),
                }
            },
            None => error::print_error_msg(format!("Account: {} doesn't exist", account)),
        }

        OtpFile::close(&store)?;

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

    #[error("no secret selected")]
    NoneSelected,
}
