pub mod add;
#[cfg(feature = "alias")]
pub mod alias;
pub mod clone;
#[cfg(feature = "clipboard")]
pub mod copy;
pub mod duplicate;
pub mod edit;
pub mod generate;
pub mod git;
pub mod housekeeping;
pub mod init;
pub mod internal;
pub mod list;
pub mod main;
pub mod r#move;
pub mod otp;
pub mod recipients;
pub mod remove;
pub mod show;
pub mod sync;
#[cfg(all(feature = "tomb", target_os = "linux"))]
pub mod tomb;

// Re-export to matcher module
#[cfg(feature = "alias")]
pub use self::alias::AliasMatcher;
#[cfg(feature = "clipboard")]
pub use self::copy::CopyMatcher;
#[cfg(all(feature = "tomb", target_os = "linux"))]
pub use self::tomb::TombMatcher;
#[rustfmt::skip]
pub use self::{
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
    otp::OtpMatcher,
    r#move::MoveMatcher,
    recipients::RecipientsMatcher,
    remove::RemoveMatcher,
    show::ShowMatcher,
    sync::SyncMatcher,

};

use clap::ArgMatches;

#[allow(single_use_lifetimes)]
pub trait Matcher<'a>: Sized {
    // Construct a new matcher instance from these argument matches.
    fn with(matches: &'a ArgMatches) -> Option<Self>;
}
