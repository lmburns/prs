#![allow(unused)]
use std::{fs, io};

use anyhow::Result;
use clap::ArgMatches;
// use data_encoding::{DecodeError, BASE32_NOPAD};
use colored::Colorize;
use std::path::Path;
use thiserror::Error;

use crate::{
    cmd::matcher::{
        main::MainMatcher,
        otp::{remove::RemoveMatcher, OtpMatcher},
        Matcher,
    },
    util::{cli, edit, error, select, sync},
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
        let matcher_remove = RemoveMatcher::with(self.cmd_matches).unwrap();

        let store = Store::open(matcher_otp.store()).map_err(Err::Store)?;
        #[cfg(all(feature = "tomb", target_os = "linux"))]
        let mut tomb = store.tomb(
            !matcher_main.verbose(),
            matcher_main.verbose(),
            matcher_main.force(),
        );

        let sync = store.sync();

        // Prepare tomb
        #[cfg(all(feature = "tomb", target_os = "linux"))]
        tomb::prepare_tomb(&mut tomb, &matcher_main).map_err(Err::Tomb)?;

        // Prepare sync
        sync::ensure_ready(&sync, matcher_remove.allow_dirty());
        if !matcher_remove.no_sync() {
            sync.prepare()?;
        }

        let mut otp_file = OtpFile::new(&store)?;

        let account_rm = if let Some(acc) = matcher_remove.account() {
            acc.to_string()
        } else if let Some(sec) = select::select_otp(&otp_file) {
            sec.name.clone()
        } else {
            OtpFile::close(&store);
            if !matcher_remove.no_sync() {
                sync.finalize("Re-encrypted OTP file")?;
            }
            return Err(anyhow::anyhow!(Err::NoneSelected));
        };

        // TODO: reverse cli prompt and prevent a sync if exiting
        // This looks really ugly
        if otp_file.get(&account_rm).is_some() {
            if cli::prompt_yes(
                format!("Remove: {}", account_rm.red().bold()).as_str(),
                Some(true),
                &matcher_main,
            ) {
                // error::quit();
                otp_file.delete(account_rm.clone());
                if let Err(e) = otp_file.save(&store) {
                    error::print_error(e);
                } else if !matcher_main.quiet() {
                    eprintln!("Successfully removed OTP account");
                }
            } else if matcher_main.verbose() {
                eprintln!("Removal cancelled");
            }
        } else {
            println!("Account does not exist");
        }

        // Encrypt and write changed plaintext
        OtpFile::close(&store)?;

        // Finalize sync
        if !matcher_remove.no_sync() {
            sync.finalize(format!("Removed OTP account: {}", account_rm))?;
        }

        // Finalize tomb
        #[cfg(all(feature = "tomb", target_os = "linux"))]
        tomb::finalize_tomb(&mut tomb, &matcher_main, true).map_err(Err::Tomb)?;

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

    #[error("no OTP selected")]
    NoneSelected,
}
