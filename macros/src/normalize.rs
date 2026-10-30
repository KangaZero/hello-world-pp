//! Text folding for lenient name matching.
//!
//! Shared verbatim with `build.rs` (via `#[path]`), so it must not use any
//! `crate::` path: the build script reads this file without the rest of the crate.

use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

/// Lowercases `text`, strips accents and treats `-`/`_` as spaces, for lenient name matching.
/// `"Québécois-French"` → `"quebecois french"`.
///
/// Lossy beyond Latin: Indic vowel signs and kana voicing marks are combining
/// marks too (`"が"` → `"か"`), so compare folded text only as a fallback.
#[must_use]
pub fn normalize_text(text: &str) -> String {
    folded_chars(text).collect()
}

/// Whether `a` and `b` are equal once folded by [`normalize_text`], without allocating.
#[must_use]
pub fn folded_eq(a: &str, b: &str) -> bool {
    folded_chars(a).eq(folded_chars(b))
}

/// The lazy folding behind [`normalize_text`] and [`folded_eq`].
///
/// Lowercases per `char`, so a word-final `Σ` folds to `σ` (not `ς` as `str::to_lowercase`
/// would); both callers share this function, so they always agree.
fn folded_chars(text: &str) -> impl Iterator<Item = char> + '_ {
    text.nfd()
        .filter(|c| !is_combining_mark(*c)) // drop every combining mark (accents, vowel signs, voicing marks)
        .map(|c| if matches!(c, '-' | '_') { ' ' } else { c })
        .flat_map(char::to_lowercase)
}
