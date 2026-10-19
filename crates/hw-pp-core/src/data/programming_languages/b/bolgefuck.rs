//! Hello-world snippet for `ProgrammingLanguageName::Bolgefuck`.
//!
//! Imported from leachim6/hello-world (`b/Bolgefuck.bf` at `a152253`),
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
    language: Bolgefuck => "Bolgefuck",
    versions: [Bolgefuck => "Bolgefuck"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bolgefuck,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bolgefuck,
            syntax: concat!(
                r"wimpmodesHpsepslpslpsops psWpsopsrpslpsdpH     //",
                output_for_hello_world!(),
                r" without moving pointers in wimpmode"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"wimpmodesHpsepslpslpsops psWpsopsrpslpsdpH     //",
                    output_for_hello_world!(),
                    r" without moving pointers in wimpmode"
                ),
            )],
        }],
    };
