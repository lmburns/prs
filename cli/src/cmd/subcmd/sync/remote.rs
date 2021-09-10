use clap::{App, Arg};

/// The sync remote command definition.
pub(crate) struct CmdRemote;

impl CmdRemote {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("remote")
            .about("Get or set git remote URL for sync")
            .arg(Arg::new("GIT_URL").about("Remote git URL to set"))
    }
}
