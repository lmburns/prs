use clap::{Command, Arg};

use crate::cmd::arg::{ArgTimeout, CmdArg};

/// The internal clipboard revert command definition.
pub(crate) struct CmdClipRevert;

impl CmdClipRevert {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("clip-revert")
            .about("Revert clipboard after timeout")
            .arg(
                Arg::new("previous-base64-stdin")
                    .long("previous-base64-stdin")
                    .help("Read previous contents from stdin as base64 line"),
            )
            .arg(ArgTimeout::build())
    }
}
