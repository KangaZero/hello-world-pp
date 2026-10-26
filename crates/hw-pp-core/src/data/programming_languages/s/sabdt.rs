//! Hello-world snippet for `ProgrammingLanguageName::Sabdt`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Sabdt>
//! - <https://esolangs.org/wiki/Sabdt>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sabdt => "Sabdt",
    versions: [Sabdt => "Sabdt"],
    extensions: [],
}

const SABDT: &str = concat!(r#"pr ""#, output_for_hello_world!(), r#"";"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sabdt,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sabdt,
            syntax: SABDT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SABDT,
                    [(
                        TokenKind::Plain,
                        concat!(r#"pr ""#, output_for_hello_world!(), r#"";"#),
                    )],
                )
            },
        }],
    };
