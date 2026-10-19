//! Hello-world snippet for `ProgrammingLanguageName::N9598`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#95-98>
//! - <https://esolangs.org/wiki/95-98>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: N9598 => "95-98",
    versions: [N9598 => "95-98"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N9598,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N9598,
            syntax: r"103-105 100-102 107-109 107-109 110-112 43-45 31-33 118-120 110-112 113-115 107-109 99-101 32-34 0-1",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"103-105 100-102 107-109 107-109 110-112 43-45 31-33 118-120 110-112 113-115 107-109 99-101 32-34 0-1",
            )],
        }],
    };
