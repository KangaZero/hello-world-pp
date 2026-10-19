//! Hello-world snippet for `ProgrammingLanguageName::Underload`.
//!
//! Imported from leachim6/hello-world (`u/Underload.ul` at `a152253`),
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
    language: Underload => "Underload",
    versions: [Underload => "Underload"],
    extensions: [Ul => "ul"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Underload,
        extensions: &[FileExtension::Ul],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Underload,
            syntax: concat!(r"(", output_for_hello_world!(), r")S"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"(", output_for_hello_world!(), r")S"),
            )],
        }],
    };
