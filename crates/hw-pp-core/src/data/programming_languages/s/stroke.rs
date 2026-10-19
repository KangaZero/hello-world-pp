//! Hello-world snippet for `ProgrammingLanguageName::Stroke`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Stroke>
//! - <https://esolangs.org/wiki/Stroke>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Stroke => "Stroke",
    versions: [Stroke => "Stroke"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stroke,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stroke,
            syntax: r"|| ||| ||||| ||||||| |||||||||| ||||||||||||| |||||||||||||||
||||||||||||||||||| |||||||||||||||||||| |||||||||||||||||||||
|||||||||||||||||||||| ||||||||||||||||||||||||
||||||||||||||||||||||||| ||||||||||||||||||||||||||
|||||||||||||||||||||||||||| |||||||||||||||||||||||||||||||||",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"|| ||| ||||| ||||||| |||||||||| ||||||||||||| |||||||||||||||
||||||||||||||||||| |||||||||||||||||||| |||||||||||||||||||||
|||||||||||||||||||||| ||||||||||||||||||||||||
||||||||||||||||||||||||| ||||||||||||||||||||||||||
|||||||||||||||||||||||||||| |||||||||||||||||||||||||||||||||",
            )],
        }],
    };
