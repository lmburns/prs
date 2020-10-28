use prs_lib::sync::{Readyness, Sync};

use crate::util::error::{quit_error, quit_error_msg, ErrorHintsBuilder};

/// Ensure the store is ready, otherwise quit.
pub fn ensure_ready(sync: &Sync) {
    let readyness = match sync.readyness() {
        Ok(readyness) => readyness,
        Err(err) => {
            quit_error(
                err.context("failed to query store sync readyness state"),
                ErrorHintsBuilder::default().git(true).build().unwrap(),
            );
        }
    };

    quit_error_msg(
        match readyness {
            Readyness::Ready | Readyness::NoSync => return,
            Readyness::Dirty => "store git repository has uncommitted changes".into(),
            Readyness::GitState(state) => {
                format!("store git repository is in unfinished state: {:?}", state)
            }
        },
        ErrorHintsBuilder::default().git(true).build().unwrap(),
    );
}