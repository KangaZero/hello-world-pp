//! Hello-world snippet for `ProgrammingLanguageName::Noflo`.
//!
//! Imported from leachim6/hello-world (`n/NoFlo.fbp` at `a152253`),
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
    language: Noflo => "NoFlo",
    versions: [Noflo => "NoFlo"],
    extensions: [Fbp => "fbp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Noflo,
        extensions: &[FileExtension::Fbp],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Noflo,
            syntax: concat!(r"'", output_for_hello_world!(), r"' -> IN Display(Output)"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"'", output_for_hello_world!(), r"' -> IN Display(Output)"),
            )],
        }],
    };
