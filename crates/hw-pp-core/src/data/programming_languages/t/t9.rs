//! Hello-world snippet for `ProgrammingLanguageName::T9`.
//!
//! Imported from the Hello World Collection (`files/T9.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/T9.txt>
//! - <http://helloworldcollection.de/#T9>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: T9 => "T9",
    versions: [T9 => "T9"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::T9,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::T9,
            syntax: concat!(
                r"How to enter ",
                output_for_hello_world!(),
                r" on a standard T9 numeric
keypad of an SMS-enabled mobile phone.
T9 predictive text has to be off.

44 33 555 555 666 0 9 666 777 555 3 11111"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"How to enter ",
                    output_for_hello_world!(),
                    r" on a standard T9 numeric
keypad of an SMS-enabled mobile phone.
T9 predictive text has to be off.

44 33 555 555 666 0 9 666 777 555 3 11111"
                ),
            )],
        }],
    };
