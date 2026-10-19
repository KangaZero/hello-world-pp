//! Hello-world snippet for `ProgrammingLanguageName::Symsyn`.
//!
//! Imported from the Hello World Collection (`files/Symsyn.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Symsyn.htm>
//! - <http://helloworldcollection.de/#Symsyn>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Symsyn => "Symsyn",
    versions: [Symsyn => "Symsyn"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Symsyn,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Symsyn,
            syntax: concat!(
                r"| ",
                output_for_hello_world!(),
                r" in Symsyn

 '",
                output_for_hello_world!(),
                r"' []"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"| ",
                    output_for_hello_world!(),
                    r" in Symsyn

 '",
                    output_for_hello_world!(),
                    r"' []"
                ),
            )],
        }],
    };
