use super::human_languages::{DialectName, HumanLanguageName, greeting_for};
use super::programming_languages::ProgrammingLanguageName;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumanLanguageCategory {
    Natural,
    Constructed,
    Signed,
    Extinct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgrammingLanguageCategory {
    Esolang,
    Scripting,
    Compiled,
    Markup,
    Historical,
}

/// Default human language used when generating hello-world output.
pub const DEFAULT_HUMAN_LANGUAGE: HumanLanguageName = HumanLanguageName::English;

/// Default dialect used when generating hello-world output.
pub const DEFAULT_DIALECT: DialectName = DialectName::AmericanEnglish;

/// Default programming language.
pub const DEFAULT_PROGRAMMING_LANGUAGE: ProgrammingLanguageName = ProgrammingLanguageName::C;

/// Greeting for the default language/dialect.
///
/// # Panics
///
/// Panics if the default language/dialect pair has no recorded greeting.
#[must_use]
pub fn default_greeting() -> &'static str {
    greeting_for(DEFAULT_HUMAN_LANGUAGE, DEFAULT_DIALECT)
        .expect("default language/dialect must resolve to a greeting")
}
