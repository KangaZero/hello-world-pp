//! Hello-world snippet for `ProgrammingLanguageName::Progfk`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#ProgFk>
//! - <https://esolangs.org/wiki/ProgFk>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Progfk => "ProgFk",
    versions: [Progfk => "ProgFk"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Progfk,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Progfk,
            syntax: r"14 11 14 11 11 14 14 11 14 11 41 14 14 11 41 41 11 14 14 11 41 41 11 14 14 11 41 41 41 41 11 41 14 14 11 11
14 11 11 11 14 14 14 11 41 41 41 14 14 11 41 41 41 41 14 14 14 11 14 11 14 14 11 41 41 11 14 14 11 14 11 11
14 11 11 14 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22
22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 27 11 11 11 11 71 11 11 11 17 11 11 11 11 71 11 11
11 17 11 11 11 11 71 11 11 11 17 11 11 11 11 71 11 11 11 17 11 11 11 11 71 11 11 11 17 11 11 11 11 71 11 11
11 17 11 11 11 11 F0",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"14 11 14 11 11 14 14 11 14 11 41 14 14 11 41 41 11 14 14 11 41 41 11 14 14 11 41 41 41 41 11 41 14 14 11 11
14 11 11 11 14 14 14 11 41 41 41 14 14 11 41 41 41 41 14 14 14 11 14 11 14 14 11 41 41 11 14 14 11 14 11 11
14 11 11 14 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22
22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 22 27 11 11 11 11 71 11 11 11 17 11 11 11 11 71 11 11
11 17 11 11 11 11 71 11 11 11 17 11 11 11 11 71 11 11 11 17 11 11 11 11 71 11 11 11 17 11 11 11 11 71 11 11
11 17 11 11 11 11 F0",
            )],
        }],
    };
