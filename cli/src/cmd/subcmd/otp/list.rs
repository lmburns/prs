use clap::App;

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, CmdArg};

/// The list command definition.
pub(crate) struct CmdList;

impl CmdList {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("list")
            .alias("l")
            .about("List available otp codes")
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
    }
}
