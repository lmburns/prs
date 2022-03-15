//! Library to the `prs` crate

// #![deny(
//     clippy::all,
//     // clippy::cargo,
//     clippy::complexity,
//     clippy::correctness,
//     clippy::nursery,
//     clippy::pedantic,
//     clippy::perf,
//     // clippy::restriction,
//     clippy::style,
//     absolute_paths_not_starting_with_crate,
//     anonymous_parameters,
//     bad_style,
//     const_err,
//     // dead_code,
//     ellipsis_inclusive_range_patterns,
//     exported_private_dependencies,
//     ill_formed_attribute_input,
//     improper_ctypes,
//     keyword_idents,
//     macro_use_extern_crate,
//     meta_variable_misuse,
//     missing_abi,
//     // missing_debug_implementations,
//     missing_docs,
//     no_mangle_generic_items,
//     non_shorthand_field_patterns,
//     noop_method_call,
//     overflowing_literals,
//     path_statements,
//     patterns_in_fns_without_body,
//     pointer_structural_match,
//     private_in_public,
//     pub_use_of_private_extern_crate,
//     semicolon_in_expressions_from_macros,
//     single_use_lifetimes,
//     trivial_casts,
//     trivial_numeric_casts,
//     unaligned_references,
//     unconditional_recursion,
//     unreachable_pub,
//     unsafe_code,
//     // unused,
//     // unused_allocation,
//     // unused_comparisons,
//     // unused_extern_crates,
//     // unused_import_braces,
//     // unused_lifetimes,
//     // unused_parens,
//     // unused_qualifications,
//     variant_size_differences,
//     while_true
// )]
// #![allow(
//     // Fill out documentation
//     // clippy::missing_docs_in_private_items,
//
//     // Find this problem
//     clippy::pattern_type_mismatch,
//
//     // ?
//     clippy::redundant_pub_crate,
//
//     clippy::as_conversions,
//     clippy::blanket_clippy_restriction_lints,
//     clippy::cast_possible_truncation,
//     clippy::cast_sign_loss,
//     clippy::cognitive_complexity,
//     clippy::create_dir,
//     clippy::doc_markdown,
//     clippy::else_if_without_else,
//     clippy::exhaustive_enums,
//     clippy::exhaustive_structs,
//     clippy::expect_used,
//     clippy::exit,
//     clippy::implicit_return,
//     clippy::indexing_slicing,
//     clippy::integer_arithmetic,
//     clippy::integer_division,
//     clippy::mod_module_files,
//     clippy::multiple_inherent_impl,
//     clippy::separated_literal_suffix,
//     clippy::shadow_reuse,
//     clippy::shadow_same,
//     clippy::shadow_unrelated,
//     clippy::similar_names,
//     clippy::string_add,
//     clippy::string_slice,
//     clippy::struct_excessive_bools,
//     clippy::too_many_lines,
//     clippy::upper_case_acronyms,
//     clippy::unreachable,
//     clippy::unwrap_in_result
//     // clippy::single_match_else,
// )]
// #![cfg_attr(
//     any(test),
//     allow(
//         clippy::expect_fun_call,
//         clippy::expect_used,
//         clippy::panic,
//         clippy::panic_in_result_fn,
//         clippy::unwrap_in_result,
//         clippy::unwrap_used,
//         clippy::wildcard_enum_match_arm,
//     )
// )]

#![feature(derive_default_enum)]
pub mod crypto;
pub(crate) mod git;
#[cfg(feature = "otp")]
pub mod otp;
pub mod store;
pub mod sync;
#[cfg(all(feature = "tomb", target_os = "linux"))]
pub mod systemd_bin;
#[cfg(all(feature = "tomb", target_os = "linux"))]
pub mod tomb;
#[cfg(all(feature = "tomb", target_os = "linux"))]
pub(crate) mod tomb_bin;
pub mod types;
pub mod util;
mod vendor;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

// Re-exports
pub use crypto::{recipients::Recipients, Key};
pub use store::{Secret, Store};
pub use types::{Ciphertext, Plaintext};

use crate::crypto::{Config, Proto};
use once_cell::sync::Lazy;
use std::env;

#[cfg(feature = "otp")]
use std::path::PathBuf;

/// Default password store directory.
#[cfg(not(windows))]
pub const STORE_DEFAULT_ROOT: &str = "~/.password-store";

/// Default password store directory.
#[cfg(windows)]
pub const STORE_DEFAULT_ROOT: &str = "~\\.password-store";

/// File location to store files with OTP
#[cfg(feature = "otp")]
#[allow(clippy::declare_interior_mutable_const)]
pub const OTP_DEFUALT_FILE_LOCATION: Lazy<PathBuf> = Lazy::new(|| {
    vendor::shellexpand::full(STORE_DEFAULT_ROOT)
        .map(|val| PathBuf::from(val.to_string()))
        .unwrap_or_else(|_| {
            dirs_next::home_dir()
                .map(|d| d.join(".password-store"))
                .expect("Invalid password store directory")
        })
        .join(".otp-codes.json")
});

#[cfg(all(feature = "otp", target_os = "windows"))]
pub const OTP_DEFUALT_FILE: Lazy<String> =
    Lazy::new(|| format!("{}/.otp-codes.json", STORE_DEFAULT_ROOT));

/// File name where OTP codes are stored
#[cfg(all(not(windows), feature = "otp"))]
pub const OTP_DEFUALT_FILE: &str = ".otp-codes.json";

/// TODO: How to for windows?
/// `Fortress` UMASK
pub static STORE_UMASK: Lazy<u32> = Lazy::new(|| {
    u32::from_str_radix(
        &env::var("PASSWORD_STORE_UMASK").unwrap_or_else(|_| "077".to_owned()),
        8,
    )
    .expect("umask is not a valid octal")
});

/// Default proto config.
// TODO: remove when multiple protocols are supported.
const CONFIG: Config = Config {
    proto:   Proto::Gpg,
    gpg_tty: false,
};
