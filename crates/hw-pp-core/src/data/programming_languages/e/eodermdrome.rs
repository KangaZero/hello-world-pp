//! Hello-world snippet for `ProgrammingLanguageName::Eodermdrome`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Eodermdrome>
//! - <https://esolangs.org/wiki/Eodermdrome>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Eodermdrome => "Eodermdrome",
    versions: [Eodermdrome => "Eodermdrome"],
    extensions: [],
}

const EODERMDROME: &str = concat!(r"this (", output_for_hello_world!(), r") says... hi!");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Eodermdrome,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Eodermdrome,
            syntax: EODERMDROME,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EODERMDROME,
                    [(
                        TokenKind::Plain,
                        concat!(r"this (", output_for_hello_world!(), r") says... hi!"),
                    )],
                )
            },
        }],
    };
