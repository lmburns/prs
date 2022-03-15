use clap::{Command, Arg};
use once_cell::sync::Lazy;

use crate::util::time;

/// Default value for timer.
static TIMER_DEFAULT: Lazy<String> = Lazy::new(|| time::format_duration(prs_lib::tomb::TOMB_AUTO_CLOSE_SEC));

/// The tomb init command definition.
pub(crate) struct CmdInit;

impl CmdInit {
    pub(crate) fn build<'a>() -> Command<'a> {
        Command::new("init")
            .alias("initialize")
            .about("Initialize tomb in-place for current password store")
            .arg(
                Arg::new("timer")
                    .long("timer")
                    .short('t')
                    .alias("time")
                    .value_name("TIME")
                    .default_value(&TIMER_DEFAULT)
                    .help("Time after which to close the Tomb"),
            )
    }
}
