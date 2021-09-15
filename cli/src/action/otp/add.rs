use anyhow::Result;
use clap::ArgMatches;
use std::io;
use thiserror::Error;

use crate::{
    cmd::matcher::{
        main::MainMatcher,
        otp::{add::AddMatcher, OtpMatcher},
        Matcher,
    },
    util::{error, select, sync},
};

use prs_lib::{
    otp::{
        uri_algorithm, uri_period, uri_secret, uri_type, AccountBuilder,
        OtpFile,
    },
    Store,
};

#[cfg(all(feature = "tomb", target_os = "linux"))]
use crate::util::tomb;

/// A file completions action.
pub(crate) struct Add<'a> {
    cmd_matches: &'a ArgMatches,
}

impl<'a> Add<'a> {
    /// Construct a new OTP add action
    pub(crate) fn new(cmd_matches: &'a ArgMatches) -> Self {
        Self { cmd_matches }
    }

    /// Invoke the OTP action
    pub(crate) fn invoke(&self) -> Result<()> {
        let _span = tracing::debug_span!("invoking otp add").entered();

        // Create the command matchers
        let matcher_main = MainMatcher::with(self.cmd_matches).unwrap();
        let matcher_otp = OtpMatcher::with(self.cmd_matches).unwrap();
        let matcher_add = AddMatcher::with(self.cmd_matches).unwrap();

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
        sync::ensure_ready(&sync, matcher_add.allow_dirty());
        if !matcher_add.no_sync() {
            println!(" preparing sync: ");
            sync.prepare()?;
        }

        // TODO: remove account option if not used
        // let secret = if let Some(name) = matcher_add.name() {
        //     let path = store
        //         .normalize_secret_path(name, None, true)
        //         .map_err(Err::NormalizePath)?;
        //
        //     Secret::from(&store, path.clone())
        // } else {
        let secret =
            select::store_select_secret(&store, matcher_add.query()).ok_or(Err::NoneSelected)?;
        // };

        tracing::debug!(secret = ?secret);

        let acc = if let Some(uri) = matcher_add.uri() {
            AccountBuilder::default()
                .name(secret.name.clone())
                .path(secret.path.clone())
                .key(uri_secret(uri)?)
                .uri(Some(uri.to_owned()))
                .totp(uri_type(uri)?)
                .hash_function(uri_algorithm(uri)?)
                .counter(if uri_type(uri)? { None } else { Some(0) })
                .period(uri_period(uri)?)
                .build()
        } else {
            AccountBuilder::default()
                .name(secret.name.clone())
                .path(secret.path.clone())
                .key(matcher_add.key())
                .uri(None)
                .totp(!matcher_add.hotp())
                .hash_function(matcher_add.algorithm())
                .counter(if matcher_add.totp() { None } else { Some(0) })
                .period(matcher_add.period())
                .build()
        }
        .unwrap();

        // TODO: allow adding otp to other files
        tracing::debug!(account = ?acc);

        let mut otp_file = OtpFile::new(&store)?;
        if otp_file.get(&acc.name).is_some() {
            error::print_error_msg("Account already exists");
        } else {
            otp_file.add(acc.clone());
            match otp_file.save(&store) {
                Ok(_) => println!(),
                Err(err) => error::print_error(&err),
            }
        }
        tracing::debug!(otp_file = ?otp_file);

        // Finalize sync
        if !matcher_add.no_sync() {
            sync.finalize(format!("Added OTP account: {}", secret.name))?;
        }

        // Finalize tomb
        #[cfg(all(feature = "tomb", target_os = "linux"))]
        tomb::finalize_tomb(&mut tomb, &matcher_main, true).map_err(Err::Tomb)?;

        if !matcher_main.quiet() {
            eprintln!("Successfully added OTP account");
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub(crate) enum Err {
    #[error("I/O error: {}", _0)]
    Io(#[from] io::Error),

    #[error("failed to access password store")]
    Store(#[source] anyhow::Error),

    #[cfg(all(feature = "tomb", target_os = "linux"))]
    #[error("failed to prepare password store tomb for usage")]
    Tomb(#[source] anyhow::Error),

    #[error("no secret selected")]
    NoneSelected,
}
