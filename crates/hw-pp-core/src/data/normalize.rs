//! Text folding for lenient name matching.
//!
//! Shared verbatim with `build.rs` (via `#[path]`), so it must not use any
//! `crate::` path: the build script reads this file without the rest of the crate.

use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

/// Lowercases `text` and strips accents, for lenient name matching.
/// `"Québécois French"` → `"quebecois french"`.
///
/// Lossy beyond Latin: Indic vowel signs and kana voicing marks are combining
/// marks too (`"が"` → `"か"`), so compare folded text only as a fallback.
#[must_use]
pub fn normalize_text(text: &str) -> String {
    text.nfd()
        .filter(|c| !is_combining_mark(*c)) // drop every combining mark (accents, vowel signs, voicing marks)
        .collect::<String>()
        .to_lowercase()
}
