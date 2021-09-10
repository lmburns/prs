#![allow(unused)]
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
    util::{cli, error, secret, select, sync},
};

use prs_lib::{
    crypto::prelude::*,
    otp::{Account, HashFunction, OneTimePassword, OtpFile},
    Plaintext, Secret, Store, OTP_DEFUALT_FILE,
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
        // let name = matcher_add.name();

        // Prepare tomb
        #[cfg(all(feature = "tomb", target_os = "linux"))]
        tomb::prepare_tomb(&mut tomb, &matcher_main).map_err(Err::Tomb)?;

        // Prepare sync
        sync::ensure_ready(&sync, matcher_add.allow_dirty());
        if !matcher_add.no_sync() {
            sync.prepare()?;
        }

        let mut context = crate::crypto::context(&matcher_main)?;
        // let mut plaintext = context.decrypt_file(&secret.path).map_err(Err::Read)?;

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

        println!("name: {:?}", secret);
        tracing::debug!(secret = ?secret);

        // TODO: allow adding otp to other files
        // TODO: allow alternate paths other than default store

        // // let mut plaintext = Plaintext::empty();
        // // plaintext = matcher_add.key().into();

        let acc = Account {
            name:          secret.name.clone(),
            path:          secret.path.clone(),
            key:           matcher_add.key(),
            totp:          !matcher_add.hotp(),
            hash_function: matcher_add.algorithm_str().to_owned(),
            counter:       if matcher_add.totp() { None } else { Some(0) },
        };
        tracing::debug!(account = ?acc);

        let mut otp_file = OtpFile::new(&store)?;
        if otp_file.get(&acc.name).is_some() {
            error::print_error_msg("Account already exists");
        } else {
            otp_file.add(acc.clone());
            match otp_file.save(&store) {
                Ok(_) => println!("OTP account successfully created"),
                Err(err) => error::print_error(err),
            }
        }
        tracing::debug!(otp_file = ?otp_file);

        // Confirm if empty secret should be stored
        // if !matcher_main.force()
        //     && !matcher_add.empty()
        //     && plaintext.is_empty()
        //     && !cli::prompt_yes("Secret is empty. Add?", Some(true), &matcher_main)
        // {
        //     error::quit();
        // }

        // // Encrypt and write changed plaintext
        // // TODO: select proper recipients (use from current file?)
        // let recipients = store.recipients()?;
        // crate::crypto::context(&matcher_main)?
        //     .encrypt_file(&recipients, plaintext, &path)
        //     .map_err(Err::Write)?;

        // Finalize sync
        if !matcher_add.no_sync() {
            sync.finalize(format!("Add secret to {}", secret.name))?;
        }

        // Finalize tomb
        #[cfg(all(feature = "tomb", target_os = "linux"))]
        tomb::finalize_tomb(&mut tomb, &matcher_main, true).map_err(Err::Tomb)?;

        if !matcher_main.quiet() {
            eprintln!("Secret added");
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

    #[error("failed to normalize destination path")]
    NormalizePath(#[source] anyhow::Error),

    #[error("failed to write changed secret")]
    Write(#[source] anyhow::Error),

    #[error("no secret selected")]
    NoneSelected,
}
