//! Hello-world snippet for `ProgrammingLanguageName::Fob`.
//!
//! Imported from leachim6/hello-world (`f/Fob.fob` at `a152253`),
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
    language: Fob => "Fob",
    versions: [Fob => "Fob"],
    extensions: [Fob => "fob"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fob,
        extensions: &[FileExtension::Fob],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fob,
            syntax: concat!(r" $", output_for_hello_world!(), r"#<>"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r" $", output_for_hello_world!(), r"#<>"),
            )],
        }],
    };
