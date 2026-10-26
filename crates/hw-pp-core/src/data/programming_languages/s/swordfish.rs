//! Hello-world snippet for `ProgrammingLanguageName::Swordfish`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Swordfish>
//! - <https://esolangs.org/wiki/Swordfish>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Swordfish => "Swordfish",
    versions: [Swordfish => "Swordfish"],
    extensions: [],
}

const SWORDFISH: &str = r"Hello:, \
,,/#;,,,W
,,\!dlro/";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Swordfish,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Swordfish,
            syntax: SWORDFISH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SWORDFISH,
                    [(
                        TokenKind::Plain,
                        r"Hello:, \
,,/#;,,,W
,,\!dlro/",
                    )],
                )
            },
        }],
    };
