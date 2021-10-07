pub(crate) mod add;
#[cfg(feature = "alias")]
pub(crate) mod alias;
pub(crate) mod clone;
#[cfg(feature = "clipboard")]
pub(crate) mod copy;
pub(crate) mod duplicate;
pub(crate) mod edit;
pub(crate) mod generate;
pub(crate) mod git;
#[cfg(feature = "grep")]
pub(crate) mod grep;
pub(crate) mod housekeeping;
pub(crate) mod init;
pub(crate) mod internal;
pub(crate) mod list;
pub(crate) mod main;
pub(crate) mod r#move;
#[cfg(feature = "otp")]
pub(crate) mod otp;
pub(crate) mod recipients;
pub(crate) mod remove;
pub(crate) mod show;
pub(crate) mod sync;
#[cfg(all(feature = "tomb", target_os = "linux"))]
pub(crate) mod tomb;

// Re-export to matcher module
#[cfg(feature = "alias")]
pub(crate) use self::alias::AliasMatcher;
#[cfg(feature = "clipboard")]
pub(crate) use self::copy::CopyMatcher;
#[cfg(feature = "grep")]
pub(crate) use self::grep::GrepMatcher;
#[cfg(feature = "otp")]
pub(crate) use self::otp::OtpMatcher;
#[cfg(all(feature = "tomb", target_os = "linux"))]
pub(crate) use self::tomb::TombMatcher;
#[rustfmt::skip]
pub(crate) use self::{
    add::AddMatcher,
    clone::CloneMatcher,
    duplicate::DuplicateMatcher,
    edit::EditMatcher,
    generate::GenerateMatcher,
    git::GitMatcher,
    housekeeping::HousekeepingMatcher,
    init::InitMatcher,
    internal::InternalMatcher,
    list::ListMatcher,
    main::MainMatcher,
    r#move::MoveMatcher,
    recipients::RecipientsMatcher,
    remove::RemoveMatcher,
    show::ShowMatcher,
    sync::SyncMatcher,

};

use clap::ArgMatches;

#[allow(single_use_lifetimes)]
pub(crate) trait Matcher<'a>: Sized {
    // Construct a new matcher instance from these argument matches.
    fn with(matches: &'a ArgMatches) -> Option<Self>;
}
