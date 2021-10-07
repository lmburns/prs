use grep_searcher::{BinaryDetection, SearcherBuilder};
use termcolor::{BufferedStandardStream, ColorChoice, StandardStream};

use anyhow::Result;
use clap::ArgMatches;
use colored::{Color, Colorize};
use prs_lib::{crypto::prelude::*, Store};
use std::io;
use thiserror::Error;

#[cfg(all(feature = "tomb", target_os = "linux"))]
use crate::cmd::matcher::{grep::GrepMatcher, MainMatcher, Matcher};
#[cfg(all(feature = "tomb", target_os = "linux"))]
use crate::util::tomb;

/// List secrets action.
pub(crate) struct Grep<'a> {
    cmd_matches: &'a ArgMatches,
}

impl<'a> Grep<'a> {
    /// Construct a new list action.
    pub(crate) fn new(cmd_matches: &'a ArgMatches) -> Self {
        Self { cmd_matches }
    }

    /// Invoke the list action.
    pub(crate) fn invoke(&self) -> Result<()> {
        // Create the command matchers
        let matcher_main = MainMatcher::with(self.cmd_matches).unwrap();
        let matcher_grep = GrepMatcher::with(self.cmd_matches).unwrap();

        let store = Store::open(matcher_grep.store()).map_err(Err::Store)?;
        #[cfg(all(feature = "tomb", target_os = "linux"))]
        let mut tomb = store.tomb(
            !matcher_main.verbose(),
            matcher_main.verbose(),
            matcher_main.force(),
        );

        // Prepare tomb
        #[cfg(all(feature = "tomb", target_os = "linux"))]
        tomb::prepare_tomb(&mut tomb, &matcher_main).map_err(Err::Tomb)?;

        let mut searcher = SearcherBuilder::new()
            .binary_detection(BinaryDetection::quit(b'\x00'))
            .build();
        let matcher_regex = grep_regex::RegexMatcher::new_line_matcher(matcher_grep.search())?;
        let mut printer = grep_printer::StandardBuilder::new()
            .color_specs(grep_printer::ColorSpecs::new(&[
                "path:fg:magenta".parse()?,
                "line:fg:green".parse()?,
                "column:fg:yellow".parse()?,
                "match:fg:red".parse()?,
                "match:style:bold".parse()?,
            ]))
            .column(true)
            .heading(true)
            .build({
                if atty::is(atty::Stream::Stdout) {
                    self::StandardStreamKind::LineBuffered(StandardStream::stdout(
                        ColorChoice::Auto,
                    ))
                } else {
                    self::StandardStreamKind::BlockBuffered(BufferedStandardStream::stdout(
                        ColorChoice::Never,
                    ))
                }
            });

        for sec in store.secret_iter() {
            let path = sec.path;
            let path_str = path.display().to_string();

            let sep = path_str.rfind('/').ok_or(Err::NoSubfolder)? + 1;
            let subfolder = &path_str[store.root.display().to_string().len() + 1..sep];
            let file = &path_str[sep..path_str.rfind(".gpg").unwrap()];

            let plaintext = crate::crypto::context(&matcher_main)?
                .decrypt_file(&path)
                .map_err(Err::Read)?;

            let path_color = format!(
                "{}{}",
                subfolder.bold().underline(),
                file.color(Color::TrueColor {
                    r: 160,
                    g: 100,
                    b: 105,
                })
                .underline(),
            );

            searcher.search_slice(
                &matcher_regex,
                plaintext.unsecure_ref(),
                printer.sink_with_path(&matcher_regex, &path_color),
            )?;
        }

        // Finalize tomb
        #[cfg(all(feature = "tomb", target_os = "linux"))]
        tomb::finalize_tomb(&mut tomb, &matcher_main, false).map_err(Err::Tomb)?;

        Ok(())
    }
}

// Taken from `grep_cli` to implement only the necessary requirements for
// `grep_searcher` instead of having to depend on `grep_cli` for this simple
// feature
enum StandardStreamKind {
    LineBuffered(StandardStream),
    BlockBuffered(BufferedStandardStream),
}

impl io::Write for StandardStreamKind {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            StandardStreamKind::LineBuffered(ref mut w) => w.write(buf),
            StandardStreamKind::BlockBuffered(ref mut w) => w.write(buf),
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        match self {
            StandardStreamKind::LineBuffered(ref mut w) => w.flush(),
            StandardStreamKind::BlockBuffered(ref mut w) => w.flush(),
        }
    }
}

impl termcolor::WriteColor for StandardStreamKind {
    #[inline]
    fn supports_color(&self) -> bool {
        match self {
            StandardStreamKind::LineBuffered(ref w) => w.supports_color(),
            StandardStreamKind::BlockBuffered(ref w) => w.supports_color(),
        }
    }

    #[inline]
    fn set_color(&mut self, spec: &termcolor::ColorSpec) -> io::Result<()> {
        match self {
            StandardStreamKind::LineBuffered(ref mut w) => w.set_color(spec),
            StandardStreamKind::BlockBuffered(ref mut w) => w.set_color(spec),
        }
    }

    #[inline]
    fn reset(&mut self) -> io::Result<()> {
        match self {
            StandardStreamKind::LineBuffered(ref mut w) => w.reset(),
            StandardStreamKind::BlockBuffered(ref mut w) => w.reset(),
        }
    }
}

#[derive(Debug, Error)]
pub(crate) enum Err {
    #[error("failed to access password store")]
    Store(#[source] anyhow::Error),

    #[cfg(all(feature = "tomb", target_os = "linux"))]
    #[error("failed to prepare password store tomb for usage")]
    Tomb(#[source] anyhow::Error),

    #[error("failed to read secret")]
    Read(#[source] anyhow::Error),

    #[error("failed no subfolder")]
    NoSubfolder,
}
