//! Hello-world snippet for `ProgrammingLanguageName::Cubix`.
//!
//! Imported from leachim6/hello-world (`c/Cubix` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Cubix => "Cubix",
    versions: [Cubix => "Cubix"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cubix,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cubix,
            syntax: r#"./v.o;@?/."dlroW"S..u/"Hello""#,
            tokenized_syntax: &[(TokenKind::Plain, r#"./v.o;@?/."dlroW"S..u/"Hello""#)],
        }],
    };
