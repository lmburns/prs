use clap::App;

/// The sync init command definition.
pub(crate) struct CmdInit;

impl CmdInit {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("init")
            .alias("initialize")
            .about("Initialize sync")
    }
}
