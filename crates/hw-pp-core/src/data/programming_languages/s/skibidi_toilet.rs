//! Hello-world snippet for `ProgrammingLanguageName::SkibidiToilet`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Skibidi_Toilet>
//! - <https://esolangs.org/wiki/Skibidi_Toilet>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: SkibidiToilet => "Skibidi Toilet",
    versions: [SkibidiToilet => "Skibidi Toilet"],
    extensions: [],
}

const SKIBIDI_TOILET: &str = concat!(r#"Toilet ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SkibidiToilet,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SkibidiToilet,
            syntax: SKIBIDI_TOILET,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SKIBIDI_TOILET,
                    [(
                        TokenKind::Plain,
                        concat!(r#"Toilet ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
