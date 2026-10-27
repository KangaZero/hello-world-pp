//! Hello-world snippet for `ProgrammingLanguageName::LeftRightMarch`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Left-Right_March>
//! - <https://esolangs.org/wiki/Left-Right_March>

use crate::data::programming_languages::ProgrammingParadigm::{CellBased, Reflective};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: LeftRightMarch => "Left-Right March",
    versions: [LeftRightMarch => "Left-Right March"],
    extensions: [],
}

const LEFT_RIGHT_MARCH: &str =
    r"c09p00c32p00c39p00p00c42p00c88p00c01p00c50p00c42p00c45p00c39p00c31p00c64p00";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LeftRightMarch,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective, CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LeftRightMarch,
            syntax: LEFT_RIGHT_MARCH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LEFT_RIGHT_MARCH,
                    [(
                        TokenKind::Plain,
                        r"c09p00c32p00c39p00p00c42p00c88p00c01p00c50p00c42p00c45p00c39p00c31p00c64p00",
                    )],
                )
            },
        }],
    };
