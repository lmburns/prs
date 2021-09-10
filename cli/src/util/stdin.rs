use std::io::{self, Read};

use anyhow::Result;
use prs_lib::Plaintext;
use thiserror::Error;

/// Read file from stdin.
fn read_file(prompt: bool) -> Result<Vec<u8>> {
    if prompt {
        #[cfg(not(windows))]
        eprintln!("Enter input. Use [CTRL+D] to stop:");
        #[cfg(windows)]
        eprintln!("Enter input. Use [CTRL+Z] to stop:");
    }

    let mut data = vec![];
    io::stdin()
        .lock()
        .read_to_end(&mut data)
        .map_err(Err::Stdin)?;
    Ok(data)
}

/// Read plaintext from stdin.
pub(crate) fn read_plaintext(prompt: bool) -> Result<Plaintext> {
    Ok(read_file(prompt).map_err(Err::Plaintext)?.into())
}

#[derive(Debug, Error)]
pub(crate) enum Err {
    #[error("failed to read from stdin")]
    Stdin(#[source] io::Error),

    #[error("failed to read plaintext")]
    Plaintext(#[source] anyhow::Error),
}
