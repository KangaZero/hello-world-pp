//! Hello-world snippet for `ProgrammingLanguageName::Stroke`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Stroke>
//! - <https://esolangs.org/wiki/Stroke>

use crate::data::programming_languages::ProgrammingParadigm::{CellBased, Imperative};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Stroke => "Stroke",
    versions: [Stroke => "Stroke"],
    extensions: [],
}

const STROKE: &str = r"|| ||| ||||| ||||||| |||||||||| ||||||||||||| |||||||||||||||
||||||||||||||||||| |||||||||||||||||||| |||||||||||||||||||||
|||||||||||||||||||||| ||||||||||||||||||||||||
||||||||||||||||||||||||| ||||||||||||||||||||||||||
|||||||||||||||||||||||||||| |||||||||||||||||||||||||||||||||";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stroke,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stroke,
            syntax: STROKE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    STROKE,
                    [(
                        TokenKind::Plain,
                        r"|| ||| ||||| ||||||| |||||||||| ||||||||||||| |||||||||||||||
||||||||||||||||||| |||||||||||||||||||| |||||||||||||||||||||
|||||||||||||||||||||| ||||||||||||||||||||||||
||||||||||||||||||||||||| ||||||||||||||||||||||||||
|||||||||||||||||||||||||||| |||||||||||||||||||||||||||||||||",
                    )],
                )
            },
        }],
    };
