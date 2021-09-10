use colored::{ColoredString, Colorize};

/// Highlight the given text with a color.
pub(crate) fn highlight(msg: &str) -> ColoredString {
    msg.yellow()
}

/// Highlight the given text with an error color.
pub(crate) fn highlight_error(msg: &str) -> ColoredString {
    msg.red().bold()
}

/// Highlight the given text with an warning color.
pub(crate) fn highlight_warning(msg: &str) -> ColoredString {
    highlight(msg).bold()
}

/// Highlight the given text with an info color
pub(crate) fn highlight_info(msg: &str) -> ColoredString {
    msg.cyan()
}
