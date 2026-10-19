//! Hello-world snippet for `ProgrammingLanguageName::Asterisk`.
//!
//! Imported from the Hello World Collection (`files/Asterisk.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Asterisk.htm>
//! - <http://helloworldcollection.de/#Asterisk>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Asterisk => "Asterisk",
    versions: [Asterisk => "Asterisk"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Asterisk,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Asterisk,
            syntax: concat!(
                r";; ",
                output_for_hello_world!(),
                r" in Asterisk

exten => s,1,NoOp(",
                output_for_hello_world!(),
                r")"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r";; ",
                    output_for_hello_world!(),
                    r" in Asterisk

exten => s,1,NoOp(",
                    output_for_hello_world!(),
                    r")"
                ),
            )],
        }],
    };
