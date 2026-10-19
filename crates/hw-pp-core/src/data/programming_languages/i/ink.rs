//! Hello-world snippet for `ProgrammingLanguageName::Ink`.
//!
//! Imported from leachim6/hello-world (`i/Ink.ink` at `a152253`),
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
    language: Ink => "Ink",
    versions: [Ink => "Ink"],
    extensions: [Ink => "ink"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ink,
        extensions: &[FileExtension::Ink],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ink,
            syntax: output_for_hello_world!(),
            tokenized_syntax: &[(TokenKind::Plain, output_for_hello_world!())],
        }],
    };
