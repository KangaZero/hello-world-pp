//! Hello-world snippet for `ProgrammingLanguageName::Fueue`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Fueue>
//! - <https://esolangs.org/wiki/Fueue>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Fueue => "Fueue",
    versions: [Fueue => "Fueue"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fueue,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fueue,
            syntax: r"72 101 108 108 111 44 32 119 111 114 108 100 33 10 H",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"72 101 108 108 111 44 32 119 111 114 108 100 33 10 H",
            )],
        }],
    };
