//! Hello-world snippet for `ProgrammingLanguageName::Pxem`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Pxem>
//! - <https://esolangs.org/wiki/Pxem>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Pxem => "Pxem",
    versions: [Pxem => "Pxem"],
    extensions: [],
}

const PXEM: &str = concat!(output_for_hello_world!(), r".pxe");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pxem,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pxem,
            syntax: PXEM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PXEM,
                    [(
                        TokenKind::Plain,
                        concat!(output_for_hello_world!(), r".pxe"),
                    )],
                )
            },
        }],
    };
