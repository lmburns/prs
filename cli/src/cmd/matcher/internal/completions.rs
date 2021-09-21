use std::{fmt, io::Write, path::PathBuf};

use clap::{App, ArgMatches};
use clap_generate::generators;

use super::Matcher;
use crate::util;

/// The completions completions command matcher.
pub(crate) struct CompletionsMatcher<'a> {
    matches: &'a ArgMatches,
}

#[allow(single_use_lifetimes)]
impl<'a: 'b, 'b> CompletionsMatcher<'a> {
    /// Get the shells to generate completions for.
    pub(crate) fn shells(&'a self) -> Vec<Shell> {
        // Get the raw list of shells
        let raw = self
            .matches
            .values_of("SHELL")
            .expect("no shells were given");

        // Parse the list of shell names, deduplicate
        let mut shells: Vec<_> = raw
            .into_iter()
            .map(|name| name.trim().to_lowercase())
            .flat_map(|name| {
                if name == "all" {
                    Shell::variants().iter().map(|s| s.name().into()).collect()
                } else {
                    vec![name]
                }
            })
            .collect();
        shells.sort_unstable();
        shells.dedup();

        // Parse the shell names
        shells
            .into_iter()
            .map(|name| Shell::from_str(&name).expect("failed to parse shell name"))
            .collect()
    }

    /// The target directory to output the shell completion files to.
    pub(crate) fn output(&'a self) -> PathBuf {
        self.matches
            .value_of("output")
            .map_or_else(|| PathBuf::from("./"), PathBuf::from)
    }

    /// Whether to print completion scripts to stdout.
    pub(crate) fn stdout(&'a self) -> bool {
        self.matches.is_present("stdout")
    }

    /// Name of binary to generate completions for.
    pub(crate) fn name(&'a self) -> String {
        self.matches
            .value_of("name")
            .map_or_else(util::bin_name, |n| n.into())
    }
}

impl<'a> Matcher<'a> for CompletionsMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("internal")?
            .subcommand_matches("completions")
            .map(|matches| CompletionsMatcher { matches })
    }
}

/// Available shells.
#[derive(Copy, Clone)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum Shell {
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
}

impl Shell {
    /// List all supported shell variants.
    pub(crate) fn variants() -> &'static [Shell] {
        &[
            Shell::Bash,
            Shell::Elvish,
            Shell::Fish,
            Shell::PowerShell,
            Shell::Zsh,
        ]
    }

    /// Select shell variant from name.
    pub(crate) fn from_str(shell: &str) -> Option<Shell> {
        match shell.trim().to_ascii_lowercase().as_str() {
            "bash" => Some(Shell::Bash),
            "elvish" => Some(Shell::Elvish),
            "fish" => Some(Shell::Fish),
            "powershell" | "ps" => Some(Shell::PowerShell),
            "zsh" => Some(Shell::Zsh),
            _ => None,
        }
    }

    /// Get shell name.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Shell::Bash => "bash",
            Shell::Elvish => "elvish",
            Shell::Fish => "fish",
            Shell::PowerShell => "powershell",
            Shell::Zsh => "zsh",
        }
    }

    /// Suggested file name for completions file of current shell.
    #[allow(unused)]
    pub(crate) fn file_name(self, bin_name: &str) -> String {
        match self {
            Shell::Bash => format!("{}.bash", bin_name),
            Shell::Elvish => format!("{}.elv", bin_name),
            Shell::Fish => format!("{}.fish", bin_name),
            Shell::PowerShell => format!("_{}.ps1", bin_name),
            Shell::Zsh => format!("_{}", bin_name),
        }
    }

    /// Generate completion script.
    pub(crate) fn generate<S>(self, app: &mut App<'_>, bin_name: S, buf: &mut dyn Write)
    where
        S: Into<String>,
    {
        match self {
            Shell::Bash => clap_generate::generate::<generators::Bash, _>(app, bin_name, buf),
            Shell::Elvish => clap_generate::generate::<generators::Elvish, _>(app, bin_name, buf),
            Shell::Fish => clap_generate::generate::<generators::Fish, _>(app, bin_name, buf),
            Shell::PowerShell =>
                clap_generate::generate::<generators::PowerShell, _>(app, bin_name, buf),
            Shell::Zsh => clap_generate::generate::<generators::Zsh, _>(app, bin_name, buf),
        }
    }

    /// Generate completion script to a directory
    pub(crate) fn generate_to<S, T>(self, app: &mut App<'_>, bin_name: S, out_dir: T)
    where
        S: Into<String>,
        T: Into<std::ffi::OsString>,
    {
        match self {
            Shell::Bash =>
                clap_generate::generate_to::<generators::Bash, _, _>(app, bin_name, out_dir),
            Shell::Elvish =>
                clap_generate::generate_to::<generators::Elvish, _, _>(app, bin_name, out_dir),
            Shell::Fish =>
                clap_generate::generate_to::<generators::Fish, _, _>(app, bin_name, out_dir),
            Shell::PowerShell =>
                clap_generate::generate_to::<generators::PowerShell, _, _>(app, bin_name, out_dir),
            Shell::Zsh =>
                clap_generate::generate_to::<generators::Zsh, _, _>(app, bin_name, out_dir),
        }
        .expect("error generating completions");
    }
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
