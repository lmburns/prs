use clap::{App, Arg};

use crate::cmd::arg::{ArgTimeout, CmdArg};

/// The internal clipboard revert command definition.
pub(crate) struct CmdClipRevert;

impl CmdClipRevert {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("clip-revert")
            .about("Revert clipboard after timeout")
            .arg(
                Arg::new("previous-base64-stdin")
                    .long("previous-base64-stdin")
                    .about("Read previous contents from stdin as base64 line"),
            )
            .arg(ArgTimeout::build())
    }
}
