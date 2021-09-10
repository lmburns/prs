use clap::{App, Arg};

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, CmdArg};

/// The housekeeping sync-keys command definition.
pub(crate) struct CmdSyncKeys;

impl CmdSyncKeys {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("sync-keys")
            .alias("sync-recipients")
            .about("Sync public keys in store, import missing keys")
            .arg(
                Arg::new("no-import")
                    .long("no-import")
                    .alias("skip-import")
                    .about("Skip importing missing keys to keychain"),
            )
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
    }
}
