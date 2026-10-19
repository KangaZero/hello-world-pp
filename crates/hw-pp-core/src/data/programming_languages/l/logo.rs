//! Hello-world snippet for `ProgrammingLanguageName::Logo`.
//!
//! Imported from leachim6/hello-world (`l/Logo.lg` at `a152253`),
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
    language: Logo => "Logo",
    versions: [Logo => "Logo"],
    extensions: [Lg => "lg"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Logo,
        extensions: &[FileExtension::Lg],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Logo,
            syntax: concat!(r"print [", output_for_hello_world!(), r"]"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"print [", output_for_hello_world!(), r"]"),
            )],
        }],
    };
