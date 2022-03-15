use clap::{Command, Arg};

/// The sync remote command definition.
pub(crate) struct CmdRemote;

impl CmdRemote {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("remote")
            .about("Get or set git remote URL for sync")
            .arg(Arg::new("GIT_URL").help("Remote git URL to set"))
    }
}
