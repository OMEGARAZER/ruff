use once_cell::sync::Lazy;
use regex::Regex;
use ruff_text_size::{TextLen, TextRange, TextSize};
use rustpython_parser::ast::Location;

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};

#[violation]
pub struct BlanketTypeIgnore;

impl Violation for BlanketTypeIgnore {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Use specific rule codes when ignoring type issues")
    }
}

static BLANKET_TYPE_IGNORE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"# type:? *ignore($|\s)").unwrap());

/// PGH003 - use of blanket type ignore comments
pub fn blanket_type_ignore(range: TextRange, line: &str) -> Option<Diagnostic> {
    BLANKET_TYPE_IGNORE_REGEX.find(line).map(|m| {
        Diagnostic::new(
            BlanketTypeIgnore,
            TextRange::at(range.start(), m.as_str().text_len()),
        )
    })
}
