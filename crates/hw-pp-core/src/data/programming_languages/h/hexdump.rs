//! Hello-world snippet for `ProgrammingLanguageName::Hexdump`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Hexdump>
//! - <https://esolangs.org/wiki/Hexdump>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Hexdump => "Hexdump",
    versions: [Hexdump => "Hexdump"],
    extensions: [],
}

const HEXDUMP: &str = r"48 65 6C 6C 6F 20 57 6F 72 6C 64 21";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hexdump,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hexdump,
            syntax: HEXDUMP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HEXDUMP,
                    [(TokenKind::Plain, r"48 65 6C 6C 6F 20 57 6F 72 6C 64 21")],
                )
            },
        }],
    };
