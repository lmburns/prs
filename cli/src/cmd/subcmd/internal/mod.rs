#[cfg(feature = "clipboard")]
pub(crate) mod clip_revert;
pub(crate) mod completions;

use clap::Command;

/// The internal command definition.
pub(crate) struct CmdInternal;

impl CmdInternal {
    pub(crate) fn build<'a>() -> Command<'a> {
        #[allow(unused_mut)]
        let mut cmd = Command::new("internal")
            .about("Commands used by prs internally")
            .hide(true)
            .subcommand_required(true)
            .subcommand(completions::CmdCompletions::build());

        #[cfg(feature = "clipboard")]
        {
            cmd = cmd.subcommand(clip_revert::CmdClipRevert::build());
        }

        cmd
    }
}
