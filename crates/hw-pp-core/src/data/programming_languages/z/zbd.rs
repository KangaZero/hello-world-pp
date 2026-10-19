//! Hello-world snippet for `ProgrammingLanguageName::Zbd`.
//!
//! Imported from leachim6/hello-world (`z/ZBD` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Zbd => "ZBD",
    versions: [Zbd => "ZBD"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zbd,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zbd,
            syntax: r#" + Mr.B says "Hello Friend" "#,
            tokenized_syntax: &[(TokenKind::Plain, r#" + Mr.B says "Hello Friend" "#)],
        }],
    };
