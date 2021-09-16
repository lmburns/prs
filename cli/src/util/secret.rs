#![allow(unused)]
use colored::Colorize;
use once_cell::sync::Lazy;
use prs_lib::{Plaintext, Secret, Store};
use regex::Regex;
use std::io::Write;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Err {
    /// Regex capture failure
    #[error("failed to get regex captures")]
    RegexCaptures,
    /// UTF8 compatible error
    #[error("UTF-8 error: {0}")]
    UTF8(#[source] std::str::Utf8Error),
    /* /// IO Error
     * #[error("IO error: {0}")]
     * IO(#[source] std::io::Error) */
}

/// Secret alias recursion limit.
const SECRET_ALIAS_DEPTH: u32 = 30;

/// Regular expression to capture property names for colorizing
static PROPERTY_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)
        (?P<prop>^.*)
        (?:(:|\s=)\s?)   # prop: value OR prop = value
        (?P<value>.*$)
        ",
    )
    .unwrap()
});

/// Print the given plaintext to stdout.
pub(crate) fn print(plaintext: &Plaintext) -> Result<(), std::io::Error> {
    let mut stdout = std::io::stdout();

    stdout.write_all(plaintext.unsecure_ref())?;

    // Always finish with newline
    if let Some(&last) = plaintext.unsecure_ref().last() {
        if last != b'\n' {
            stdout.write_all(&[b'\n'])?;
        }
    }

    let _drop = stdout.flush();
    Ok(())
}

pub(crate) fn print_colored(plaintext: &Plaintext) -> Result<(), Err> {
    for (idx, line) in plaintext
        .unsecure_to_str()
        .map_err(Err::UTF8)?
        .split('\n')
        .collect::<Vec<_>>()
        .iter()
        .enumerate()
    {
        // TODO: make sure this works on all formats
        if let Some(caps) = PROPERTY_REGEX.captures(line) {
            println!(
                "{}: {}",
                caps.name("prop")
                    .ok_or(Err::RegexCaptures)?
                    .as_str()
                    .blue()
                    .bold(),
                caps.name("value")
                    .ok_or(Err::RegexCaptures)?
                    .as_str()
                    .yellow()
            );
        // Should not need to check against anything else if there are no capture groups present
        } else if idx == 0 {
            // println!("{}: {}", "Pass".blue().bold(), line.green());
            println!("{}", line.green());
        // Fallback
        } else {
            println!("{}", line.yellow());
        }
    }

    Ok(())
}

/// Show full secret name if query was partial.
///
/// This notifies the user on what exact secret is selected when only part of
/// the secret name is entered. This is useful for when a partial (short) query
/// selects the wrong secret.
pub(crate) fn print_name(query: Option<String>, secret: &Secret, store: &Store, quiet: bool) {
    // If quiet or query matches exact name, do not print it
    if quiet || query.map_or(false, |q| secret.name.eq(&q)) {
        return;
    }

    // Show secret with alias target if available
    if let Some(alias) = resolve_alias(secret, store) {
        eprintln!(
            "{}: {} {} {}",
            "Secret".red(),
            secret.name.magenta().bold(),
            "=>".bold(),
            alias.name.yellow()
        );
    } else {
        eprintln!("{}: {}", "Secret".red().underline(), secret.name.magenta().bold().underline());
    }
}

/// Resolve secret that is aliased.
///
/// This find the target alias if the given secret is an alias. This uses
/// recursive searching. If the secret is not an alias, `None` is returned.
fn resolve_alias(secret: &Secret, store: &Store) -> Option<Secret> {
    fn f(secret: &Secret, store: &Store, depth: u32) -> Option<Secret> {
        assert!(
            depth < SECRET_ALIAS_DEPTH,
            "failed to resolve secret alias target, recursion limit reached"
        );
        match secret.alias_target(store) {
            Ok(s) => f(&s, store, depth + 1),
            Err(_) if depth > 0 => Some(secret.clone()),
            Err(_) => None,
        }
    }
    f(secret, store, 0)
}
