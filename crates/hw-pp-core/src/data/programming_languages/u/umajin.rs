//! Hello-world snippet for `ProgrammingLanguageName::Umajin`.
//!
//! Imported from leachim6/hello-world (`u/Umajin.u` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/u/Umajin.u>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Umajin => "Umajin",
    versions: [Umajin => "Umajin"],
    extensions: [U => "u"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Umajin,
        extensions: &[FileExtension::U],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Umajin,
            syntax: concat!(
                r"println '",
                output_for_hello_world!(),
                r"'
"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"println '",
                    output_for_hello_world!(),
                    r"'
"
                ),
            )],
        }],
    };
