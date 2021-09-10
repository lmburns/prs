pub(crate) mod allow_dirty;
pub(crate) mod no_sync;
pub(crate) mod property;
pub(crate) mod query;
pub(crate) mod store;
pub(crate) mod timeout;

use clap::{Arg, ArgMatches};

// Re-export to arg module
pub(crate) use self::{
    allow_dirty::ArgAllowDirty, no_sync::ArgNoSync, property::ArgProperty, query::ArgQuery,
    store::ArgStore, timeout::ArgTimeout,
};

/// A generic trait, for a reusable command argument struct.
/// The `CmdArgFlag` and `CmdArgOption` traits further specify what kind of
/// argument this is.
pub(crate) trait CmdArg {
    /// Get the argument name that is used as main identifier.
    fn name() -> &'static str;

    /// Build the argument.
    fn build<'a>() -> Arg<'a>;
}

/// This `CmdArg` specification defines that this argument may be tested as
/// flag. This will allow to test whether the flag is present in the given
/// matches.
pub(crate) trait CmdArgFlag: CmdArg {
    /// Check whether the argument is present in the given matches.
    #[allow(single_use_lifetimes)]
    #[allow(unused_lifetimes)]
    fn is_present<'a>(matches: &ArgMatches) -> bool {
        matches.is_present(Self::name())
    }
}

/// This `CmdArg` specification defines that this argument may be tested as
/// option. This will allow to fetch the value of the argument.
pub(crate) trait CmdArgOption<'a>: CmdArg {
    /// The type of the argument value.
    type Value;

    /// Get the argument value.
    #[allow(single_use_lifetimes)]
    #[allow(unused_lifetimes)]
    fn value<'b: 'a>(matches: &'a ArgMatches) -> Self::Value;

    /// Get the raw argument value, as a string reference.
    #[allow(single_use_lifetimes)]
    #[allow(unused_lifetimes)]
    fn value_raw<'b: 'a>(matches: &'a ArgMatches) -> Option<&'a str> {
        matches.value_of(Self::name())
    }

    /// Get the raw argument values, as a string reference.
    #[allow(single_use_lifetimes)]
    #[allow(unused_lifetimes)]
    fn values_raw<'b: 'a>(matches: &'a ArgMatches) -> Option<clap::Values<'a>> {
        matches.values_of(Self::name())
    }
}
