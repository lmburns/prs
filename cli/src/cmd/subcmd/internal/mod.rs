#[cfg(feature = "clipboard")]
pub(crate) mod clip_revert;
pub(crate) mod completions;

use clap::{App, AppSettings};

/// The internal command definition.
pub(crate) struct CmdInternal;

impl CmdInternal {
    pub(crate) fn build<'a>() -> App<'a> {
        #[allow(unused_mut)]
        let mut cmd = App::new("internal")
            .about("Commands used by prs internally")
            .setting(AppSettings::Hidden)
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(completions::CmdCompletions::build());

        #[cfg(feature = "clipboard")]
        {
            cmd = cmd.subcommand(clip_revert::CmdClipRevert::build());
        }

        cmd
    }
}
