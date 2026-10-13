use crate::data::programming_languages::get_hello_world_syntax_for;

use super::human_languages::{
    DialectName, HumanLanguageName, ScriptName, get_human_language_hello_world_for,
};
use super::programming_languages::ProgrammingLanguageName;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgrammingLanguageCategory {
    Esolang,
    Scripting,
    Compiled,
    Markup,
    Historical,
}

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
