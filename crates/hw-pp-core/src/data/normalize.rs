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
        .filter(|c| !is_combining_mark(*c)) // keep everything except the accent marks
        .collect::<String>()
        .to_lowercase()
}

// `normalize_text` is crate-private, so it is tested here rather than in `tests/`.
// `build.rs` never compiles `cfg(test)`, so sharing this file is unaffected.
#[cfg(test)]
mod tests {
    use super::normalize_text;

    #[test]
    fn strips_accents_and_lowercases() {
        for (input, expected) in [
            ("Québécois French", "quebecois french"), // acute
            ("Bokmål", "bokmal"),                     // ring above
            ("Čeština", "cestina"),                   // caron
            ("Română", "romana"),                     // breve
            ("Türkçe", "turkce"),                     // diaeresis + cedilla
            ("Ñandú", "nandu"),                       // tilde + acute
            ("ÉCOLE", "ecole"),                       // uppercase, non-ASCII
            ("Tiếng Việt", "tieng viet"),             // two stacked marks on one letter
            ("Ẹ n lẹ, Ayé!", "e n le, aye!"),         // dot below; punctuation kept
            ("Ἑλληνική", "ελληνικη"),                 // Greek breathing + accent
        ] {
            assert_eq!(normalize_text(input), expected, "{input:?}");
        }
    }

    #[test]
    fn treats_precomposed_and_decomposed_input_alike() {
        // "é" as one code point vs "e" followed by a combining acute accent
        assert_eq!(
            normalize_text("Qu\u{e9}bec"),
            normalize_text("Que\u{301}bec")
        );
        assert_eq!(normalize_text("Que\u{301}bec"), "quebec");
    }

    #[test]
    fn keeps_letters_that_are_not_letter_plus_accent() {
        // These are distinct letters, not base letter + mark, so NFD leaves them alone.
        assert_eq!(normalize_text("Søren"), "søren");
        assert_eq!(normalize_text("Đà Nẵng"), "đa nang");
        assert_eq!(normalize_text("日本語"), "日本語");
    }

    #[test]
    fn is_lossy_for_non_latin_combining_marks() {
        // Documented limitation: vowel signs and voicing marks are combining marks
        // too, so they are stripped. Matching stays symmetric (both sides fold the
        // same way), but different words can fold to the same text.
        assert_eq!(normalize_text("हिन्दी"), "हनद");
        assert_eq!(normalize_text("が"), "か");
    }
}
