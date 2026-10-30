//! Hello-world snippet for `ProgrammingLanguageName::Deadfish`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Deadfish>
//! - <https://esolangs.org/wiki/Deadfish>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Deadfish => "Deadfish",
    versions: [Deadfish => "Deadfish"],
    extensions: [],
}

const DEADFISH: &str = r"iiisdsiiiiiiiioiiiiiiiiiiiiiiiiiiiiiiiiiiiiioiiiiiiiooiiio
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddoddddddddddddo
dddddddddddddddddddddsddoddddddddoiiioddddddoddddddddo
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddo";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Deadfish,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Deadfish,
            syntax: DEADFISH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DEADFISH,
                    [(
                        TokenKind::Plain,
                        r"iiisdsiiiiiiiioiiiiiiiiiiiiiiiiiiiiiiiiiiiiioiiiiiiiooiiio
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddoddddddddddddo
dddddddddddddddddddddsddoddddddddoiiioddddddoddddddddo
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddo",
                    )],
                )
            },
        }],
    };
