//! Hello-world snippet for `ProgrammingLanguageName::Acidic`.
//!
//! Imported from leachim6/hello-world (`a/ACIDIC.acidic` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ACIDIC.acidic>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Acidic => "ACIDIC",
    versions: [Acidic => "ACIDIC"],
    extensions: [Acidic => "acidic"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Acidic,
        extensions: &[FileExtension::Acidic],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Acidic,
            syntax: concat!(
                output_for_hello_world!(),
                r"
wP"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    output_for_hello_world!(),
                    r"
wP"
                ),
            )],
        }],
    };
