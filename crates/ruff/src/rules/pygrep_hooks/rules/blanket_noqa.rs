use once_cell::sync::Lazy;
use regex::Regex;
use ruff_text_size::{TextRange, TextSize};

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};

#[violation]
pub struct BlanketNOQA;

impl Violation for BlanketNOQA {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Use specific rule codes when using `noqa`")
    }
}

static BLANKET_NOQA_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)# noqa($|\s|:[^ ])").unwrap());

/// PGH004 - use of blanket noqa comments
pub fn blanket_noqa(range: TextRange, line: &str) -> Option<Diagnostic> {
    BLANKET_NOQA_REGEX.find(line).map(|m| {
        Diagnostic::new(
            BlanketNOQA,
            TextRange::new(
                range.start() + TextSize::try_from(m.start()).unwrap(),
                range.start() + TextSize::try_from(m.end()).unwrap(),
            ),
        )
    })
}
