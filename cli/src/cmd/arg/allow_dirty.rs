use clap::Arg;

use super::{CmdArg, CmdArgFlag};

/// The allow-dirty argument.
pub(crate) struct ArgAllowDirty {}

impl CmdArg for ArgAllowDirty {
    fn name() -> &'static str {
        "allow-dirty"
    }

    fn build<'b>() -> Arg<'b> {
        Arg::new("allow-dirty")
            .long("allow-dirty")
            .short('d')
            .alias("dirty")
            .alias("sync-allow-dirty")
            .alias("sync-dirty")
            .global(true)
            .help("Allow commit and sync on dirty store repository")
    }
}

impl CmdArgFlag for ArgAllowDirty {}
