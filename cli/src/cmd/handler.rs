use clap::{
    crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches, ColorChoice,
    Command,
};

use super::{
    matcher::{self, Matcher},
    subcmd,
};

use std::env;

/// CLI argument handler.
#[derive(Debug)]
pub(crate) struct Handler {
    /// The CLI matches.
    matches: ArgMatches,
}

impl<'a> Handler {
    /// Build the application CLI definition.
    pub(crate) fn build() -> Command<'a> {
        // Build the CLI application definition
        let app = Command::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .propagate_version(true)
            .disable_help_subcommand(true)
            .disable_colored_help(false)
            .color(
                env::var_os("NO_COLOR")
                    .is_none()
                    .then(|| ColorChoice::Auto)
                    .unwrap_or(ColorChoice::Never),
            )
            .arg(
                Arg::new("force")
                    .long("force")
                    .short('f')
                    .global(true)
                    .help("Force the action, ignore warnings"),
            )
            .arg(
                Arg::new("no-interact")
                    .long("no-interact")
                    .short('I')
                    .alias("no-interactive")
                    .alias("non-interactive")
                    .global(true)
                    .help("Not interactive, do not prompt"),
            )
            .arg(
                Arg::new("yes")
                    .long("yes")
                    .short('y')
                    .alias("assume-yes")
                    .global(true)
                    .help("Assume yes for prompts"),
            )
            .arg(
                Arg::new("quiet")
                    .long("quiet")
                    .short('q')
                    .global(true)
                    .help("Produce output suitable for logging and automation"),
            )
            .arg(
                Arg::new("verbose")
                    .long("verbose")
                    .short('v')
                    .multiple_occurrences(true)
                    .global(true)
                    .takes_value(false)
                    .help("Enable verbose information and logging"),
            )
            .arg(
                Arg::new("gpg-tty")
                    .long("gpg-tty")
                    .global(true)
                    .help("Instruct GPG to ask passphrase in TTY rather than pinentry"),
            )
            .subcommand(subcmd::CmdAdd::build())
            .subcommand(subcmd::CmdClone::build())
            .subcommand(subcmd::CmdDuplicate::build())
            .subcommand(subcmd::CmdEdit::build())
            .subcommand(subcmd::CmdGenerate::build())
            .subcommand(subcmd::CmdGit::build())
            .subcommand(subcmd::CmdGrep::build())
            .subcommand(subcmd::CmdHousekeeping::build())
            .subcommand(subcmd::CmdInit::build())
            .subcommand(subcmd::CmdInternal::build())
            .subcommand(subcmd::CmdList::build())
            .subcommand(subcmd::CmdMove::build())
            .subcommand(subcmd::CmdOtp::build())
            .subcommand(subcmd::CmdRecipients::build())
            .subcommand(subcmd::CmdRemove::build())
            .subcommand(subcmd::CmdShow::build())
            .subcommand(subcmd::CmdSync::build());

        #[cfg(feature = "alias")]
        let app = app.subcommand(subcmd::CmdAlias::build());

        #[cfg(feature = "clipboard")]
        let app = app.subcommand(subcmd::CmdCopy::build());

        #[cfg(all(feature = "tomb", target_os = "linux"))]
        let app = app.subcommand(subcmd::CmdTomb::build());

        app
    }

    /// Parse CLI arguments.
    pub(crate) fn parse() -> Handler {
        Handler {
            matches: Handler::build().get_matches(),
        }
    }

    /// Get the raw matches.
    pub(crate) fn matches(&'a self) -> &'a ArgMatches {
        &self.matches
    }

    /// Get the add sub command, if matched.
    pub(crate) fn add(&'a self) -> Option<matcher::AddMatcher> {
        matcher::AddMatcher::with(&self.matches)
    }

    /// Get the alias sub command, if matched.
    #[cfg(feature = "alias")]
    pub(crate) fn alias(&'a self) -> Option<matcher::AliasMatcher> {
        matcher::AliasMatcher::with(&self.matches)
    }

    /// Get the clone sub command, if matched.
    pub(crate) fn clone(&'a self) -> Option<matcher::CloneMatcher> {
        matcher::CloneMatcher::with(&self.matches)
    }

    /// Get the copy sub command, if matched.
    #[cfg(feature = "clipboard")]
    pub(crate) fn copy(&'a self) -> Option<matcher::CopyMatcher> {
        matcher::CopyMatcher::with(&self.matches)
    }

    /// Get the duplicate sub command, if matched.
    pub(crate) fn duplicate(&'a self) -> Option<matcher::DuplicateMatcher> {
        matcher::DuplicateMatcher::with(&self.matches)
    }

    /// Get the edit sub command, if matched.
    pub(crate) fn edit(&'a self) -> Option<matcher::EditMatcher> {
        matcher::EditMatcher::with(&self.matches)
    }

    /// Get the generate sub command, if matched.
    pub(crate) fn generate(&'a self) -> Option<matcher::GenerateMatcher> {
        matcher::GenerateMatcher::with(&self.matches)
    }

    /// Get the git sub command, if matched.
    pub(crate) fn git(&'a self) -> Option<matcher::GitMatcher> {
        matcher::GitMatcher::with(&self.matches)
    }

    /// Get the grep sub command, if matched.
    pub(crate) fn grep(&'a self) -> Option<matcher::GrepMatcher> {
        matcher::GrepMatcher::with(&self.matches)
    }

    /// Get the housekeeping sub command, if matched.
    pub(crate) fn housekeeping(&'a self) -> Option<matcher::HousekeepingMatcher> {
        matcher::HousekeepingMatcher::with(&self.matches)
    }

    /// Get the init sub command, if matched.
    pub(crate) fn init(&'a self) -> Option<matcher::InitMatcher> {
        matcher::InitMatcher::with(&self.matches)
    }

    /// Get the internal sub command, if matched.
    pub(crate) fn internal(&'a self) -> Option<matcher::InternalMatcher> {
        matcher::InternalMatcher::with(&self.matches)
    }

    /// Get the list sub command, if matched.
    pub(crate) fn list(&'a self) -> Option<matcher::ListMatcher> {
        matcher::ListMatcher::with(&self.matches)
    }

    /// Get the otp sub command, if matched.
    pub(crate) fn otp(&'a self) -> Option<matcher::OtpMatcher> {
        matcher::OtpMatcher::with(&self.matches)
    }

    /// Get the move sub command, if matched.
    pub(crate) fn r#move(&'a self) -> Option<matcher::MoveMatcher> {
        matcher::MoveMatcher::with(&self.matches)
    }

    /// Get the recipients sub command, if matched.
    pub(crate) fn recipients(&'a self) -> Option<matcher::RecipientsMatcher> {
        matcher::RecipientsMatcher::with(&self.matches)
    }

    /// Get the remove sub command, if matched.
    pub(crate) fn remove(&'a self) -> Option<matcher::RemoveMatcher> {
        matcher::RemoveMatcher::with(&self.matches)
    }

    /// Get the show sub command, if matched.
    pub(crate) fn show(&'a self) -> Option<matcher::ShowMatcher> {
        matcher::ShowMatcher::with(&self.matches)
    }

    /// Get the sync sub command, if matched.
    pub(crate) fn sync(&'a self) -> Option<matcher::SyncMatcher> {
        matcher::SyncMatcher::with(&self.matches)
    }

    /// Get the tomb sub command, if matched.
    #[cfg(all(feature = "tomb", target_os = "linux"))]
    pub(crate) fn tomb(&'a self) -> Option<matcher::TombMatcher> {
        matcher::TombMatcher::with(&self.matches)
    }
}
