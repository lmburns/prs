pub mod copy;
pub mod delete;
pub mod duplicate;
pub mod edit;
pub mod generate;
pub mod git;
pub mod init;
pub mod list;
pub mod main;
pub mod r#move;
pub mod new;
pub mod recipients;
pub mod show;

// Re-export to matcher module
pub use self::copy::CopyMatcher;
pub use self::delete::DeleteMatcher;
pub use self::duplicate::DuplicateMatcher;
pub use self::edit::EditMatcher;
pub use self::generate::GenerateMatcher;
pub use self::git::GitMatcher;
pub use self::init::InitMatcher;
pub use self::list::ListMatcher;
pub use self::main::MainMatcher;
pub use self::new::NewMatcher;
pub use self::r#move::MoveMatcher;
pub use self::recipients::RecipientsMatcher;
pub use self::show::ShowMatcher;

use clap::ArgMatches;

pub trait Matcher<'a>: Sized {
    // Construct a new matcher instance from these argument matches.
    fn with(matches: &'a ArgMatches) -> Option<Self>;
}
