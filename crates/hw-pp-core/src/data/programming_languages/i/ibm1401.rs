//! Hello-world snippet for `ProgrammingLanguageName::Ibm1401`.
//!
//! Imported from leachim6/hello-world (`i/IBM1401` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Ibm1401 => "IBM1401",
    versions: [Ibm1401 => "IBM1401"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ibm1401,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ibm1401,
            syntax: r",008015,022029,036043,050054,055062,063065,066077/333/M0762502F1.HELLO
WORLD",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r",008015,022029,036043,050054,055062,063065,066077/333/M0762502F1.HELLO
WORLD",
            )],
        }],
    };
