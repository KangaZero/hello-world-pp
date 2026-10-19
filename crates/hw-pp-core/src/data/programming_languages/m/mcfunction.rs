//! Hello-world snippet for `ProgrammingLanguageName::Mcfunction`.
//!
//! Imported from leachim6/hello-world (PR #1670 `m/Mcfunction.mcfunction`),
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
    language: Mcfunction => "Mcfunction",
    versions: [Mcfunction => "Mcfunction"],
    extensions: [Mcfunction => "mcfunction"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mcfunction,
        extensions: &[FileExtension::Mcfunction],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mcfunction,
            syntax: concat!(r"say ", output_for_hello_world!()),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"say ", output_for_hello_world!()),
            )],
        }],
    };
