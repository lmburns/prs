use clap::ArgMatches;

use super::Matcher;
use crate::util::error::{quit_error, ErrorHints};
use anyhow::anyhow;

/// The tomb open command matcher.
pub struct OpenMatcher<'a> {
    matches: &'a ArgMatches,
}

impl OpenMatcher<'_> {
    /// The time to automatically close.
    pub fn timer(&self) -> Option<u32> {
        let time = self.matches.value_of("timer").unwrap_or("0");
        match crate::util::time::parse_duration(time) {
            Ok(0) => None,
            Ok(time) => Some(time as u32),
            Err(err) => quit_error(&anyhow!(err), ErrorHints::default()),
        }
    }
}

impl<'a> Matcher<'a> for OpenMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("tomb")?
            .subcommand_matches("open")
            .map(|matches| OpenMatcher { matches })
    }
}
