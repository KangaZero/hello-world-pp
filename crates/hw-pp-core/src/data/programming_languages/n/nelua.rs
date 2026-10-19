//! Hello-world snippet for `ProgrammingLanguageName::Nelua`.
//!
//! Imported from leachim6/hello-world (`n/Nelua.nelua` at `a152253`),
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
    language: Nelua => "Nelua",
    versions: [Nelua => "Nelua"],
    extensions: [Nelua => "nelua"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nelua,
        extensions: &[FileExtension::Nelua],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nelua,
            syntax: concat!(r"print '", output_for_hello_world!(), r"'"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"print '", output_for_hello_world!(), r"'"),
            )],
        }],
    };
