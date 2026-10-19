//! Hello-world snippet for `ProgrammingLanguageName::Slashes`.
//!
//! Imported from leachim6/hello-world (`s/Slashes.slashes` at `a152253`),
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
    language: Slashes => "Slashes",
    versions: [Slashes => "Slashes"],
    extensions: [Slashes => "slashes"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Slashes,
        extensions: &[FileExtension::Slashes],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Slashes,
            syntax: output_for_hello_world!(),
            tokenized_syntax: &[(TokenKind::Plain, output_for_hello_world!())],
        }],
    };
