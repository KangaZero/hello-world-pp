//! Hello-world snippet for `ProgrammingLanguageName::Mumps`.
//!
//! Imported from leachim6/hello-world (`m/Mumps.m` at `a152253`),
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
    language: Mumps => "Mumps",
    versions: [Mumps => "Mumps"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mumps,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mumps,
            syntax: concat!(r#" w ""#, output_for_hello_world!(), r#"",!"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#" w ""#, output_for_hello_world!(), r#"",!"#),
            )],
        }],
    };
