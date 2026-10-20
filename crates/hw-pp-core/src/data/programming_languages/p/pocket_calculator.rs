//! Hello-world snippet for `ProgrammingLanguageName::PocketCalculator`.
//!
//! Imported from the Hello World Collection (`files/Pocket&nbsp;Calculator` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Pocket%26nbsp%3BCalculator>
//! - <http://helloworldcollection.de/#Pocket%C2%A0Calculator>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: PocketCalculator => "Pocket Calculator",
    versions: [PocketCalculator => "Pocket Calculator"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PocketCalculator,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PocketCalculator,
            syntax: concat!(
                output_for_hello_world!(),
                r" for standard pocket calculators (7-segment display).
Type in and turn calculator upside down.

0.7734"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    output_for_hello_world!(),
                    r" for standard pocket calculators (7-segment display).
Type in and turn calculator upside down.

0.7734"
                ),
            )],
        }],
    };
