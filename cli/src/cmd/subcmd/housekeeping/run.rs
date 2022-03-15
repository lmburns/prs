use clap::Command;

use crate::cmd::arg::{ArgAllowDirty, ArgNoSync, CmdArg};

/// The housekeeping run command definition.
pub(crate) struct CmdRun;

impl CmdRun {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("run")
            .about("Run housekeeping tasks")
            .arg(ArgAllowDirty::build())
            .arg(ArgNoSync::build())
    }
}
