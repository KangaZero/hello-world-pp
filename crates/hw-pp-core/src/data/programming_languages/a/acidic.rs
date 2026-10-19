//! Hello-world snippet for `ProgrammingLanguageName::Acidic`.
//!
//! Imported from leachim6/hello-world (`a/ACIDIC.acidic` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Acidic => "ACIDIC",
    versions: [Acidic => "ACIDIC"],
    extensions: [Acidic => "acidic"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Acidic,
        extensions: &[FileExtension::Acidic],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Acidic,
            syntax: concat!(
                output_for_hello_world!(),
                r"
wP"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    output_for_hello_world!(),
                    r"
wP"
                ),
            )],
        }],
    };
