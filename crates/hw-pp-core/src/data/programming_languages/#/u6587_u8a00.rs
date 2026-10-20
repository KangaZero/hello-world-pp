//! Hello-world snippet for `ProgrammingLanguageName::U6587U8a00`.
//!
//! Imported from leachim6/hello-world (`#/文言.wy` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%E6%96%87%E8%A8%80.wy>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Esolang],
        families: &[],
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
