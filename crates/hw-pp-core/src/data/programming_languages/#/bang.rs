//! Hello-world snippet for `ProgrammingLanguageName::Bang`.
//!
//! Imported from leachim6/hello-world (`#/!` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%21>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Bang => "!",
    versions: [Bang => "!"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bang,
            syntax: concat!(
                r"!-
#_",
                output_for_hello_world!(),
                r"
-!"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"!-
#_",
                    output_for_hello_world!(),
                    r"
-!"
                ),
            )],
        }],
    };
