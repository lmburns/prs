use anyhow::Result;
use clap::{Arg, ArgMatches};
use thiserror::Error;

use super::{CmdArg, CmdArgOption};

/// The timeout argument.
pub(crate) struct ArgTimeout {}

impl ArgTimeout {
    #[cfg(feature = "clipboard")]
    #[allow(unused_lifetimes)]
    pub(crate) fn value_or_default<'a, 'b: 'a>(matches: &'a ArgMatches) -> Result<u64> {
        Self::value(matches).unwrap_or(Ok(crate::CLIPBOARD_TIMEOUT))
    }
}

impl CmdArg for ArgTimeout {
    fn name() -> &'static str {
        "timeout"
    }

    fn build<'b>() -> Arg<'b> {
        Arg::new("timeout")
            .long("timeout")
            .short('t')
            .alias("time")
            .alias("seconds")
            .alias("second")
            .value_name("SECONDS")
            .global(true)
            .about("Timeout after which to clear clipboard")
    }
}

impl<'a> CmdArgOption<'a> for ArgTimeout {
    type Value = Option<Result<u64>>;

    #[allow(unused_lifetimes)]
    fn value<'b: 'a>(matches: &'a ArgMatches) -> Self::Value {
        Self::value_raw(matches).map(|t| t.parse().map_err(|err| Err::Parse(err).into()))
    }
}

#[derive(Debug, Error)]
pub(crate) enum Err {
    #[error("failed to parse timeout as seconds")]
    Parse(#[source] std::num::ParseIntError),
}
