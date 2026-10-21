use crate::data::programming_languages::get_hello_world_syntax_for;

use super::human_languages::{
    DialectName, HumanLanguageName, ScriptName, get_human_language_hello_world_for,
};
use super::macros::output_for_hello_world;
use super::programming_languages::ProgrammingLanguageName;
use unicode_segmentation::UnicodeSegmentation;

pub const DEFAULT_HUMAN_LANGUAGE: HumanLanguageName = HumanLanguageName::English;
pub const DEFAULT_HUMAN_LANGUAGE_DIALECT: DialectName = DialectName::AmericanEnglish;
pub const DEFAULT_SCRIPT: ScriptName = ScriptName::Latin;
pub const DEFAULT_PROGRAMMING_LANGUAGE: ProgrammingLanguageName = ProgrammingLanguageName::C;
// pub const DEFAULT_ENCODING:

/// Greeting for the default human language/dialect/script.
///
/// # Panics
///
/// Panics if the default language/dialect pair has no recorded greeting.
#[must_use]
pub fn get_default_human_language_hello_world() -> &'static str {
    get_human_language_hello_world_for(
        Some(DEFAULT_HUMAN_LANGUAGE),
        Some(DEFAULT_HUMAN_LANGUAGE_DIALECT),
        Some(DEFAULT_SCRIPT),
    )
    .expect("default human language/dialect/script must resolve to a greeting")
}

/// Hello-world snippet for the default programming language and its first version.
///
/// # Panics
///
/// Panics if the default programming language has no recorded snippet.
#[must_use]
pub fn get_default_programming_language_hello_world() -> &'static str {
    get_hello_world_syntax_for(Some(DEFAULT_PROGRAMMING_LANGUAGE), None, None)
        .expect("default programming language must resolve to a snippet")
}

/// Punctuation marks that appear in the recorded human-language greetings:
/// Latin, CJK (`，！、`), the Arabic comma `،`, the Tibetan shad `།` and the
/// Javanese pada lingsa `꧈`.
///
/// Deliberately excludes:
/// - the apostrophe `'`, which is a letter in Klingon (`qo'`) and part of
///   contractions like `G'day`;
/// - braille cells such as `⠂` (comma) and `⠖` (exclamation mark), whose
///   meaning depends on the braille code and their position in a word;
/// - the Tibetan tsheg `་`, which separates syllables inside a word.
pub const HELLO_WORLD_PUNCTUATION: &[char] = &[',', '!', '¡', '，', '！', '、', '،', '།', '꧈'];

/// `text` without any [`HELLO_WORLD_PUNCTUATION`].
///
/// ```
/// use hw_pp_core::data::common::remove_punctuation_from_str;
/// assert_eq!(remove_punctuation_from_str("¡Hola, Mundo!"), "Hola Mundo");
/// ```
#[must_use]
pub fn remove_punctuation_from_str(text: &str) -> String {
    text.replace(HELLO_WORLD_PUNCTUATION, "")
}

/// `text` reversed by grapheme cluster, so accents stay on their letters.
///
/// ```
/// use hw_pp_core::data::common::reverse_str;
/// assert_eq!(reverse_str("Hello, World!"), "!dlroW ,olleH");
/// assert_eq!(reverse_str("Olá"), "álO"); // `á` stays one character, even when decomposed
/// ```
#[must_use]
pub fn reverse_str(text: &str) -> String {
    text.graphemes(true).rev().collect()
}

/// `syntax` with its `"Hello, World!"` swapped for `hello_world_to_replace_with`.
///
/// - Forwards message: replaced as is.
/// - Reversed message (`"!dlroW ,olleH"`, e.g. Befunge-93): replaced with the greeting reversed.
/// - Neither, or both (which is ambiguous): `syntax` is returned unchanged, because the
///   message is encoded (e.g. Brainfuck) and cannot be replaced.
///
/// The greeting is not escaped for the target language (`G'day` breaks `print('...')`).
///
/// ```
/// use hw_pp_core::data::common::replace_default_hello_world_string_with;
/// let german = replace_default_hello_world_string_with(r#"print("Hello, World!")"#, "Hallo, Welt!");
/// assert_eq!(german, r#"print("Hallo, Welt!")"#);
/// let befunge = replace_default_hello_world_string_with(r#""!dlroW ,olleH">:#,_@"#, "Hallo, Welt!");
/// assert_eq!(befunge, r#""!tleW ,ollaH">:#,_@"#);
/// ```
#[must_use]
pub fn replace_default_hello_world_string_with(
    syntax: &str,
    hello_world_to_replace_with: &str,
) -> String {
    let hello_world = output_for_hello_world!();
    let hello_world_reversed = reverse_str(hello_world);

    match (
        syntax.contains(hello_world),
        syntax.contains(&hello_world_reversed),
    ) {
        (true, false) => syntax.replace(hello_world, hello_world_to_replace_with),
        (false, true) => syntax.replace(
            &hello_world_reversed,
            &reverse_str(hello_world_to_replace_with),
        ),
        (false, false) | (true, true) => syntax.to_owned(),
    }
}
