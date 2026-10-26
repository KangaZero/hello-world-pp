//! Hello-world snippet for `ProgrammingLanguageName::Unless`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#--Unless>
//! - <https://esolangs.org/wiki/--Unless>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Unless => "--Unless",
    versions: [Unless => "--Unless"],
    extensions: [],
}

const UNLESS: &str = r"R:
    Helloworld.c";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Unless,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Unless,
            syntax: UNLESS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    UNLESS,
                    [(
                        TokenKind::Plain,
                        r"R:
    Helloworld.c",
                    )],
                )
            },
        }],
    };
