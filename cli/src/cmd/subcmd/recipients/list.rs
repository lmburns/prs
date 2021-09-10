use clap::App;

/// The recipient list command definition.
pub(crate) struct CmdList;

impl CmdList {
    pub(crate) fn build<'a>() -> App<'a> {
        App::new("list")
            .alias("ls")
            .alias("l")
            .about("List store recipients")
    }
}
