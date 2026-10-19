//! Hello-world snippet for `ProgrammingLanguageName::U0642U0644U0628`.
//!
//! Imported from leachim6/hello-world (`#/قلب` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: U0642U0644U0628 => "قلب",
    versions: [U0642U0644U0628 => "قلب"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U0642U0644U0628,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U0642U0644U0628,
            syntax: "\u{202b}(قول \"مرحبا يا عالم\")",
            tokenized_syntax: &[(TokenKind::Plain, "\u{202b}(قول \"مرحبا يا عالم\")")],
        }],
    };
