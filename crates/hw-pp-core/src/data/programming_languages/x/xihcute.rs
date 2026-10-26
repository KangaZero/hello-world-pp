//! Hello-world snippet for `ProgrammingLanguageName::Xihcute`.
//!
//! Imported from leachim6/hello-world (`x/Xihcute` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/Xihcute>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Xihcute => "Xihcute",
    versions: [Xihcute => "Xihcute"],
    extensions: [],
}

const XIHCUTE: &str = concat!(r"92*1+ytotm0!52*~ble", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xihcute,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xihcute,
            syntax: XIHCUTE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XIHCUTE,
                    [(
                        TokenKind::Plain,
                        concat!(r"92*1+ytotm0!52*~ble", output_for_hello_world!()),
                    )],
                )
            },
        }],
    };
