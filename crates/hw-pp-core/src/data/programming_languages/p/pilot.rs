//! Hello-world snippet for `ProgrammingLanguageName::Pilot`.
//!
//! Imported from the Hello World Collection (`files/PILOT.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/PILOT.htm>
//! - <http://helloworldcollection.de/#PILOT>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Pilot => "PILOT",
    versions: [Pilot => "PILOT"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pilot,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pilot,
            syntax: concat!(
                r"R:",
                output_for_hello_world!(),
                r" in PILOT
T:",
                output_for_hello_world!()
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"R:",
                    output_for_hello_world!(),
                    r" in PILOT
T:",
                    output_for_hello_world!()
                ),
            )],
        }],
    };
