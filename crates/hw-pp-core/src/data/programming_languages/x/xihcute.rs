//! Hello-world snippet for `ProgrammingLanguageName::Xihcute`.
//!
//! Imported from leachim6/hello-world (`x/Xihcute` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Xihcute => "Xihcute",
    versions: [Xihcute => "Xihcute"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xihcute,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xihcute,
            syntax: concat!(r"92*1+ytotm0!52*~ble", output_for_hello_world!()),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"92*1+ytotm0!52*~ble", output_for_hello_world!()),
            )],
        }],
    };
