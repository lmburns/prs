use std::fs;
use std::io;

use anyhow::Result;
use clap::ArgMatches;
use thiserror::Error;

use crate::cmd::matcher::{internal::completions::CompletionsMatcher, main::MainMatcher, Matcher};

/// A file completions action.
pub(crate) struct Completions<'a> {
    cmd_matches: &'a ArgMatches,
}

impl<'a> Completions<'a> {
    /// Construct a new completions action.
    pub(crate) fn new(cmd_matches: &'a ArgMatches) -> Self {
        Self { cmd_matches }
    }

    /// Invoke the completions action.
    pub(crate) fn invoke(&self) -> Result<()> {
        // Create the command matchers
        let matcher_main = MainMatcher::with(self.cmd_matches).unwrap();
        let matcher_completions = CompletionsMatcher::with(self.cmd_matches).unwrap();

        // Obtian shells to generate completions for, build application definition
        let shells = matcher_completions.shells();
        let dir = matcher_completions.output();
        let quiet = matcher_main.quiet();
        let mut app = crate::cmd::handler::Handler::build();

        // If the directory does not exist yet, attempt to create it
        if !dir.is_dir() {
            fs::create_dir_all(&dir).map_err(Error::CreateOutputDir)?;
        }

        // Generate completions
        for shell in shells {
            if !quiet {
                eprint!(
                    "Generating completions for {}...",
                    format!("{}", shell).to_lowercase()
                );
            }
            if matcher_completions.stdout() {
                shell.generate(&mut app, matcher_completions.name(), &mut std::io::stdout());
            } else {
                shell.generate_to(&mut app, matcher_completions.name(), &dir)?;
            }
            if !quiet {
                eprintln!(" done.");
            }
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub(crate) enum Error {
    /// An error occurred while creating the output directory.
    #[error("failed to create output directory, it doesn't exist")]
    CreateOutputDir(#[source] io::Error),

    // /// An error occurred while writing completion scripts to a file.
    // #[error("failed to write completion script to file")]
    // Write(#[source] io::Error),
}
