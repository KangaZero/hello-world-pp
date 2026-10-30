//! Hello-world snippet for `ProgrammingLanguageName::Pilot`.
//!
//! Imported from the Hello World Collection (`files/PILOT.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/PILOT.htm>
//! - <http://helloworldcollection.de/#PILOT>
//! - <https://www.wikidata.org/wiki/Q2044212> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Pilot => "PILOT",
    versions: [Pilot => "PILOT"],
    extensions: [],
}

const PILOT: &str = concat!(
    r"R:",
    output_for_hello_world!(),
    r" in PILOT
T:",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pilot,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pilot,
            syntax: PILOT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PILOT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"R:",
                            output_for_hello_world!(),
                            r" in PILOT
T:",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };
