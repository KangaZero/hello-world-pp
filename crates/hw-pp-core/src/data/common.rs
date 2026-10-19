use crate::data::programming_languages::get_hello_world_syntax_for;

use super::human_languages::{
    DialectName, HumanLanguageName, ScriptName, get_human_language_hello_world_for,
};
use super::macros::output_for_hello_world;
use super::programming_languages::ProgrammingLanguageName;
use thiserror::Error;

pub const DEFAULT_HUMAN_LANGUAGE: HumanLanguageName = HumanLanguageName::English;
pub const DEFAULT_HUMAN_LANGUAGE_DIALECT: DialectName = DialectName::AmericanEnglish;
pub const DEFAULT_SCRIPT: ScriptName = ScriptName::Latin;
pub const DEFAULT_PROGRAMMING_LANGUAGE: ProgrammingLanguageName = ProgrammingLanguageName::C;
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

/// Punctuation marks that appear in the recorded human-language greetings.
///
/// Deliberately excludes:
/// - the apostrophe `'`, which is a letter in Klingon (`qo'`) and part of
///   contractions like `G'day`;
/// - braille cells such as `⠂` (comma) and `⠖` (exclamation mark), whose
///   meaning depends on the braille code and their position in a word.
pub const HELLO_WORLD_PUNCTUATION: &[char] = &[',', '!', '¡', '，', '！', '、'];

/// Removes [`HELLO_WORLD_PUNCTUATION`] from a greeting.
#[must_use]
pub fn remove_punctuation_from_hello_world(hello_world: &str) -> String {
    hello_world.replace(HELLO_WORLD_PUNCTUATION, "")
}
/// Error returned when a snippet has no hello-world message to replace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("snippet does not contain {:?}", output_for_hello_world!())]
pub struct HelloWorldNotFoundError;

/// Replaces every `"Hello, World!"` in `syntax` with `hello_world_to_replace_with`.
///
/// The greeting is not escaped for the target language (`G'day` breaks `print('...')`).
///
/// ```
/// use hw_pp_core::data::common::replace_default_hello_world_string_with;
/// let german = replace_default_hello_world_string_with(r#"print("Hello, World!")"#, "Hallo, Welt!");
/// assert_eq!(german, Ok(r#"print("Hallo, Welt!")"#.to_owned()));
/// ```
///
/// # Errors
///
/// [`HelloWorldNotFoundError`] if `syntax` does not contain it (e.g. Brainfuck).
pub fn replace_default_hello_world_string_with(
    syntax: &str,
    hello_world_to_replace_with: &str,
) -> Result<String, HelloWorldNotFoundError> {
    let hello_world = output_for_hello_world!();
    if !syntax.contains(hello_world) {
        return Err(HelloWorldNotFoundError);
    }
    Ok(syntax.replace(hello_world, hello_world_to_replace_with))
}

pub use super::normalize::normalize_text;
