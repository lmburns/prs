use anyhow::Result;
use clap::ArgMatches;
// use data_encoding::{DecodeError, BASE32_NOPAD};
use colored::Colorize;
use thiserror::Error;

use crate::{
    cmd::matcher::{
        main::MainMatcher,
        otp::{remove::RemoveMatcher, OtpMatcher},
        Matcher,
    },
    util::{cli, error, select, sync},
};

use prs_lib::{otp::OtpFile, Store};

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
        } else {
            let sec = select::select_otp(&otp_file).ok_or(Err::NoneSelected)?;
            sec.name.clone()
        };

        if otp_file.get(&account_rm).is_some() {
            if !cli::prompt_yes(
                format!("Remove: {}", account_rm.red().bold()).as_str(),
                Some(true),
                &matcher_main,
            ) {
                error::quit();
            }
            otp_file.delete(&account_rm);
            if let Err(e) = otp_file.save(&store) {
                error::print_error(&e);
            }
        } else {
            println!("Account does not exist");
        }

        // Finalize sync
        if !matcher_remove.no_sync() {
            sync.finalize(format!("Removed OTP account: {}", account_rm))?;
        }

        // Finalize tomb
        #[cfg(all(feature = "tomb", target_os = "linux"))]
        tomb::finalize_tomb(&mut tomb, &matcher_main, true).map_err(Err::Tomb)?;

        if !matcher_main.quiet() {
            eprintln!("Successfully removed OTP account");
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

    #[error("no OTP selected")]
    NoneSelected,
}
