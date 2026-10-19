//! Hello-world snippet for `ProgrammingLanguageName::Zbd`.
//!
//! Imported from leachim6/hello-world (`z/ZBD` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/ZBD>

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
