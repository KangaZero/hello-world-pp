//! Hello-world snippet for `ProgrammingLanguageName::Readlink`.
//!
//! Imported from leachim6/hello-world (`r/Readlink.readlink` at `a152253`),
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
    language: Readlink => "Readlink",
    versions: [Readlink => "Readlink"],
    extensions: [Readlink => "readlink"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Readlink,
        extensions: &[FileExtension::Readlink],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Readlink,
            syntax: output_for_hello_world!(),
            tokenized_syntax: &[(TokenKind::Plain, output_for_hello_world!())],
        }],
    };
