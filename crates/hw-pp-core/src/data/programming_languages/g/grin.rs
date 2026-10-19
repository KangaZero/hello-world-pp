//! Hello-world snippet for `ProgrammingLanguageName::Grin`.
//!
//! Imported from leachim6/hello-world (`g/Grin.grin` at `a152253`),
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
    language: Grin => "Grin",
    versions: [Grin => "Grin"],
    extensions: [Grin => "grin"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Grin,
        extensions: &[FileExtension::Grin],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Grin,
            syntax: concat!(r"(", output_for_hello_world!(), r")"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"(", output_for_hello_world!(), r")"),
            )],
        }],
    };
