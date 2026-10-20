//! Hello-world snippet for `ProgrammingLanguageName::U0642U0644U0628`.
//!
//! Imported from leachim6/hello-world (`#/قلب` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%D9%82%D9%84%D8%A8>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U0642U0644U0628,
            syntax: "\u{202b}(قول \"مرحبا يا عالم\")",
            tokenized_syntax: &[(TokenKind::Plain, "\u{202b}(قول \"مرحبا يا عالم\")")],
        }],
    };
