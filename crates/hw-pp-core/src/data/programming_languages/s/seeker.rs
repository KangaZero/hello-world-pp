//! Hello-world snippet for `ProgrammingLanguageName::Seeker`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Seeker>
//! - <https://esolangs.org/wiki/Seeker>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Seeker => "Seeker",
    versions: [Seeker => "Seeker"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Seeker,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Seeker,
            syntax: r"!chr.@chr:72.$chr.@chr:101.$chr.@chr:108.$chr.@chr:111.$chr.@chr:32.$chr.@chr:119.$chr.@chr:114.$chr.@chr:100.$chr.@chr:33.$chr._",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"!chr.@chr:72.$chr.@chr:101.$chr.@chr:108.$chr.@chr:111.$chr.@chr:32.$chr.@chr:119.$chr.@chr:114.$chr.@chr:100.$chr.@chr:33.$chr._",
            )],
        }],
    };
