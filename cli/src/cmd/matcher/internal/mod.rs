#[cfg(feature = "clipboard")]
pub(crate) mod clip_revert;
pub(crate) mod completions;

use clap::ArgMatches;

use super::Matcher;

/// The internal matcher.
pub(crate) struct InternalMatcher<'a> {
    root: &'a ArgMatches,
    _matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> InternalMatcher<'a> {
    /// Get the internal clipboard revert sub command, if matched.
    #[cfg(feature = "clipboard")]
    pub(crate) fn clip_revert(&'a self) -> Option<clip_revert::ClipRevertMatcher> {
        clip_revert::ClipRevertMatcher::with(&self.root)
    }

    /// Get the internal completions generator sub command, if matched.
    pub(crate) fn completions(&'a self) -> Option<completions::CompletionsMatcher> {
        completions::CompletionsMatcher::with(&self.root)
    }
}

impl<'a> Matcher<'a> for InternalMatcher<'a> {
    fn with(root: &'a ArgMatches) -> Option<Self> {
        root.subcommand_matches("internal")
            .map(|matches| InternalMatcher {
                root,
                _matches: matches,
            })
    }
}
