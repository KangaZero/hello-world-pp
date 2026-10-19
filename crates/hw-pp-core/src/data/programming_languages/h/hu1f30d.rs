//! Hello-world snippet for `ProgrammingLanguageName::HU1f30d`.
//!
//! Imported from leachim6/hello-world (`h/H🌍` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/H%F0%9F%8C%8D>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: HU1f30d => "H🌍",
    versions: [HU1f30d => "H🌍"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::HU1f30d,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::HU1f30d,
            syntax: r"hw",
            tokenized_syntax: &[(TokenKind::Plain, r"hw")],
        }],
    };
