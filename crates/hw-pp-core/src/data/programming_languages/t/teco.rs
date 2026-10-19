//! Hello-world snippet for `ProgrammingLanguageName::Teco`.
//!
//! Imported from the Hello World Collection (`files/TECO.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/TECO.txt>
//! - <http://helloworldcollection.de/#TECO>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Teco => "TECO",
    versions: [Teco => "TECO"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Teco,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Teco,
            syntax: concat!(
                r"!",
                output_for_hello_world!(),
                r" in TECO
!The $ symbol below wouldn't actually be a printing character -
!it's the [escape] character, \u001b!
FT",
                output_for_hello_world!(),
                r"$"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"!",
                    output_for_hello_world!(),
                    r" in TECO
!The $ symbol below wouldn't actually be a printing character -
!it's the [escape] character, \u001b!
FT",
                    output_for_hello_world!(),
                    r"$"
                ),
            )],
        }],
    };
