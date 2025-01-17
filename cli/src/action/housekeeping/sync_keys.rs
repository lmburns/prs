use anyhow::Result;
use clap::ArgMatches;
use prs_lib::{
    crypto::{self, store::ImportResult},
    Store,
};
use thiserror::Error;

#[cfg(all(feature = "tomb", target_os = "linux"))]
use crate::util::tomb;
use crate::{
    cmd::matcher::{
        housekeeping::{sync_keys::SyncKeysMatcher, HousekeepingMatcher},
        MainMatcher, Matcher,
    },
    util::sync,
};

/// A housekeeping sync-keys action.
pub(crate) struct SyncKeys<'a> {
    cmd_matches: &'a ArgMatches,
}

impl<'a> SyncKeys<'a> {
    /// Construct a new sync-keys action.
    pub(crate) fn new(cmd_matches: &'a ArgMatches) -> Self {
        Self { cmd_matches }
    }

    /// Invoke the sync-keys action.
    pub(crate) fn invoke(&self) -> Result<()> {
        // Create the command matchers
        let matcher_main = MainMatcher::with(self.cmd_matches).unwrap();
        let matcher_housekeeping = HousekeepingMatcher::with(self.cmd_matches).unwrap();
        let matcher_sync_keys = SyncKeysMatcher::with(self.cmd_matches).unwrap();

        if matcher_main.verbose() {
            eprintln!("Syncing public key files in store with selected recipients...");
        }

        let store = Store::open(matcher_housekeeping.store()).map_err(Err::Store)?;
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
        sync::ensure_ready(&sync, matcher_sync_keys.allow_dirty());
        if !matcher_sync_keys.no_sync() {
            sync.prepare()?;
        }

        // Import missing keys into keychain
        if !matcher_sync_keys.no_import() {
            import_missing_keys(&store, matcher_main.quiet(), matcher_main.verbose())
                .map_err(Err::ImportKeys)?;
        }

        // Sync public key files in store
        let recipients = store.recipients().map_err(Err::Load)?;
        crypto::store::store_sync_public_key_files(&store, recipients.keys())?;

        // Finalize sync
        if !matcher_sync_keys.no_sync() {
            sync.finalize("Sync keys")?;
        }

        // Finalize tomb
        #[cfg(all(feature = "tomb", target_os = "linux"))]
        tomb::finalize_tomb(&mut tomb, &matcher_main, true).map_err(Err::Tomb)?;

        if !matcher_main.quiet() {
            eprintln!("Keys synced");
        }

        Ok(())
    }
}

/// Import missing keys from store to keychain.
fn import_missing_keys(store: &Store, quiet: bool, verbose: bool) -> Result<()> {
    if verbose {
        eprintln!("Importing missing public keys from recipients...");
    }

    // Import keys, report results
    for result in crypto::store::import_missing_keys_from_store(store)? {
        match result {
            ImportResult::Imported(fingerprint) => {
                if !quiet {
                    eprintln!("Imported key to keychain: {}", fingerprint);
                }
            }
            ImportResult::Unavailable(fingerprint) => eprintln!(
                "Cannot import missing public key, not available in store: {}",
                fingerprint
            ),
        }
    }

    Ok(())
}

#[derive(Debug, Error)]
pub(crate) enum Err {
    #[error("failed to access password store")]
    Store(#[source] anyhow::Error),

    #[cfg(all(feature = "tomb", target_os = "linux"))]
    #[error("failed to prepare password store tomb for usage")]
    Tomb(#[source] anyhow::Error),

    #[error("failed to load store recipients")]
    Load(#[source] anyhow::Error),

    #[error("failed to import public keys to keychain")]
    ImportKeys(#[source] anyhow::Error),
}
