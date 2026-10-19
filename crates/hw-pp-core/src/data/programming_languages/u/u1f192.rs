//! Hello-world snippet for `ProgrammingLanguageName::U1f192`.
//!
//! Imported from leachim6/hello-world (`#/🆒` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: U1f192 => "🆒",
    versions: [U1f192 => "🆒"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U1f192,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U1f192,
            syntax: r"💬🔤👋🗺️🔤",
            tokenized_syntax: &[(TokenKind::Plain, r"💬🔤👋🗺️🔤")],
        }],
    };
