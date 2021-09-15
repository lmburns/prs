#![deny(
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::pedantic,
    clippy::style
)]
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    bad_style,
    const_err,
    dead_code,
    keyword_idents,
    improper_ctypes,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    no_mangle_generic_items,
    non_shorthand_field_patterns,
    noop_method_call,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    pointer_structural_match,
    private_in_public,
    semicolon_in_expressions_from_macros,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unaligned_references,
    unconditional_recursion,
    unreachable_pub,
    unsafe_code,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_parens,
    unused_qualifications,
    variant_size_differences,
    while_true
)]
#![allow(
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
//     clippy::shadow_reuse,
//     clippy::too_many_lines,
//     clippy::doc_markdown,
//     clippy::single_match_else
)]

mod action;
mod cmd;
mod crypto;
mod util;
mod vendor;

use std::{error, io, env};

use anyhow::Result;
use clap::{crate_description, crate_name, crate_version};
use prs_lib::Store;

use crate::{
    cmd::{
        matcher::{MainMatcher, Matcher},
        Handler,
    },
    util::{
        error::{quit, quit_error, ErrorHints},
        style,
    },
};

use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::{
    fmt::{
        format::{Format, Pretty},
        SubscriberBuilder,
    },
    EnvFilter, FmtSubscriber,
};

/// Tracing environment variable
const LOG_ENV: &str = "PRS_LOG";

/// Clipboard timeout in seconds.
#[cfg(feature = "clipboard")]
const CLIPBOARD_TIMEOUT: u64 = 20;

fn error_and_exit<E: error::Error>(err: E) -> ! {
    let _dropped = get_subscriber().try_init();
    tracing::error!("{}", err);
    std::process::exit(1);
}

type Subscriber = SubscriberBuilder<Pretty, Format<Pretty, ()>, LevelFilter, fn() -> io::Stdout>;
fn get_subscriber() -> Subscriber {
    FmtSubscriber::builder()
        .pretty()
        .with_ansi(true)
        .with_level(true)
        .with_target(false)
        .without_time()
        .with_max_level(if cfg!(debug_assertions) {
            Level::DEBUG
        } else {
            Level::INFO
        })
}

fn main() {
    // Do not use colored output on Windows
    #[cfg(windows)]
    colored::control::set_override(false);

    if env::var_os("NO_COLOR").is_some() {
        colored::control::set_override(false);
    }

    let subscriber = get_subscriber();
    match env::var_os(LOG_ENV) {
        Some(_) => match EnvFilter::try_from_env(LOG_ENV) {
            Err(err) => error_and_exit(err),
            Ok(filter) => subscriber
                .with_env_filter(filter)
                .with_filter_reloading()
                .init(),
        },
        None => subscriber.init(),
    };

    // Parse CLI arguments
    let cmd_handler = Handler::parse();

    // Invoke the proper action
    if let Err(err) = invoke_action(&cmd_handler) {
        quit_error(err, ErrorHints::default());
    };
}

/// Invoke the proper action based on the CLI input.
///
/// If no proper action is selected, the program will quit with an error
/// message.
fn invoke_action(handler: &Handler) -> Result<()> {
    if handler.add().is_some() {
        return action::add::Add::new(handler.matches()).invoke();
    }

    #[cfg(feature = "alias")]
    if handler.alias().is_some() {
        return action::alias::Alias::new(handler.matches()).invoke();
    }

    if handler.clone().is_some() {
        return action::clone::Clone::new(handler.matches()).invoke();
    }

    #[cfg(feature = "clipboard")]
    if handler.copy().is_some() {
        return action::copy::Copy::new(handler.matches()).invoke();
    }

    if handler.duplicate().is_some() {
        return action::duplicate::Duplicate::new(handler.matches()).invoke();
    }

    if handler.edit().is_some() {
        return action::edit::Edit::new(handler.matches()).invoke();
    }

    if handler.generate().is_some() {
        return action::generate::Generate::new(handler.matches()).invoke();
    }

    if handler.git().is_some() {
        return action::git::Git::new(handler.matches()).invoke();
    }

    if handler.grep().is_some() {
        return action::grep::Grep::new(handler.matches()).invoke();
    }

    if handler.housekeeping().is_some() {
        return action::housekeeping::Housekeeping::new(handler.matches()).invoke();
    }

    if handler.r#move().is_some() {
        return action::r#move::Move::new(handler.matches()).invoke();
    }

    if handler.init().is_some() {
        return action::init::Init::new(handler.matches()).invoke();
    }

    if handler.internal().is_some() {
        return action::internal::Internal::new(handler.matches()).invoke();
    }

    if handler.list().is_some() {
        return action::list::List::new(handler.matches()).invoke();
    }

    if handler.otp().is_some() {
        return action::otp::Otp::new(handler.matches()).invoke();
    }

    if handler.recipients().is_some() {
        return action::recipients::Recipients::new(handler.matches()).invoke();
    }

    if handler.remove().is_some() {
        return action::remove::Remove::new(handler.matches()).invoke();
    }

    if handler.show().is_some() {
        return action::show::Show::new(handler.matches()).invoke();
    }

    if handler.sync().is_some() {
        return action::sync::Sync::new(handler.matches()).invoke();
    }

    #[cfg(all(feature = "tomb", target_os = "linux"))]
    if handler.tomb().is_some() {
        return action::tomb::Tomb::new(handler.matches()).invoke();
    }

    // Get the main matcher
    let matcher_main = MainMatcher::with(handler.matches()).unwrap();
    if !matcher_main.quiet() {
        print_main_info(&matcher_main);
    }

    Ok(())
}

/// Print the main info, shown when no subcommands were supplied.
#[allow(clippy::missing_panics_doc)]
pub fn print_main_info(matcher_main: &MainMatcher) -> ! {
    // Get the name of the used executable
    let bin = util::bin_name();

    // Attempt to load default store
    let store = Store::open(prs_lib::STORE_DEFAULT_ROOT).ok();
    let has_sync = store.as_ref().map_or(false, |s| s.sync().is_init());

    // Print the main info
    eprintln!("{} {}", crate_name!(), crate_version!());
    eprintln!("Usage: {} [FLAGS] <SUBCOMMAND> ...", bin);
    eprintln!(crate_description!());
    eprintln!();

    #[allow(clippy::branches_sharing_code)]
    if let Ok(store) = Store::open(prs_lib::STORE_DEFAULT_ROOT) {
        // Hint user to add ourselves as recipient if it doesn't have recipient we own
        let we_own_any_recipient = store
            .recipients()
            .and_then(|recip| prs_lib::crypto::recipients::contains_own_secret_key(&recip))
            .unwrap_or(false);
        if !we_own_any_recipient {
            let config = crate::crypto::config(matcher_main);
            let system_has_secret = prs_lib::crypto::util::has_private_key(&config).unwrap_or(true);
            if system_has_secret {
                eprintln!("Add your own key as recipient or generate a new one:");
            } else {
                eprintln!("Generate and add a new recipient key for yourself:");
            }
            if system_has_secret {
                eprintln!(
                    "    {}",
                    style::highlight(&format!("{} recipients add --secret", bin))
                );
            }
            eprintln!(
                "    {}",
                style::highlight(&format!("{} recipients generate", bin))
            );
            eprintln!();
        }

        // Hint show/copy commands if user has secret
        let has_secret = store.secret_iter().next().is_some();
        if has_secret {
            #[cfg(not(feature = "clipboard"))]
            eprintln!("Show a secret:");
            #[cfg(feature = "clipboard")]
            eprintln!("Show or copy a secret:");
            eprintln!("    {}", style::highlight(&format!("{} show [NAME]", bin)));
            #[cfg(feature = "clipboard")]
            eprintln!("    {}", style::highlight(&format!("{} copy [NAME]", bin)));
            eprintln!();
        }

        // Hint add/edit/remove commands if store has recipient we own
        if we_own_any_recipient {
            eprintln!("Generate, add, edit or remove secrets:");
            eprintln!(
                "    {}",
                style::highlight(&format!("{} generate <NAME>", bin))
            );
            eprintln!("    {}", style::highlight(&format!("{} add <NAME>", bin)));
            eprintln!("    {}", style::highlight(&format!("{} edit [NAME]", bin)));
            eprintln!(
                "    {}",
                style::highlight(&format!("{} remove [NAME]", bin))
            );
            eprintln!();
        }

        // Hint about sync
        if has_sync {
            eprintln!("Sync your password store:");
            eprintln!("    {}", style::highlight(&format!("{} sync", bin)));
            eprintln!();
        } else {
            eprintln!("Enable sync for your password store:");
            eprintln!("    {}", style::highlight(&format!("{} sync init", bin)));
            eprintln!();
        }
    } else {
        eprintln!("Initialize a new password store or clone an existing one:");
        eprintln!("    {}", style::highlight(&format!("{} init", bin)));
        eprintln!(
            "    {}",
            style::highlight(&format!("{} clone <GIT_URL>", bin))
        );
        eprintln!();
    }

    eprintln!("Show all subcommands, features and other help:");
    eprintln!(
        "    {}",
        style::highlight(&format!("{} help [SUBCOMMAND]", bin))
    );

    quit()
}
