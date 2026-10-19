//! Hello-world snippet for `ProgrammingLanguageName::Brainz`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#BrainZ>
//! - <https://esolangs.org/wiki/BrainZ>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Brainz => "BrainZ",
    versions: [Brainz => "BrainZ"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brainz,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brainz,
            syntax: r" FF 09 F0 9C F3 CC 44 45 04 CC A4 C2 20 8A 09 20 4A A0 7F 81 03 C0 92 04 50 22 07 81 55 50 2A AA A0 49 81 38 10",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r" FF 09 F0 9C F3 CC 44 45 04 CC A4 C2 20 8A 09 20 4A A0 7F 81 03 C0 92 04 50 22 07 81 55 50 2A AA A0 49 81 38 10",
            )],
        }],
    };
