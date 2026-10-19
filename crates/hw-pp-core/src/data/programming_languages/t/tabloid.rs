//! Hello-world snippet for `ProgrammingLanguageName::Tabloid`.
//!
//! Imported from leachim6/hello-world (`t/Tabloid` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Tabloid => "Tabloid",
    versions: [Tabloid => "Tabloid"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tabloid,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tabloid,
            syntax: concat!(
                r"YOU WON'T WANT TO MISS '",
                output_for_hello_world!(),
                r"'

PLEASE LIKE AND SUBSCRIBE"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"YOU WON'T WANT TO MISS '",
                    output_for_hello_world!(),
                    r"'

PLEASE LIKE AND SUBSCRIBE"
                ),
            )],
        }],
    };
