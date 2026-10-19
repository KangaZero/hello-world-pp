//! Hello-world snippet for `ProgrammingLanguageName::U6587U8a00`.
//!
//! Imported from leachim6/hello-world (`#/文言.wy` at `a152253`),
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
    language: U6587U8a00 => "文言",
    versions: [U6587U8a00 => "文言"],
    extensions: [Wy => "wy"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U6587U8a00,
        extensions: &[FileExtension::Wy],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U6587U8a00,
            syntax: concat!(
                r"吾有一言。曰「「",
                output_for_hello_world!(),
                r"」」。書之。"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"吾有一言。曰「「",
                    output_for_hello_world!(),
                    r"」」。書之。"
                ),
            )],
        }],
    };
