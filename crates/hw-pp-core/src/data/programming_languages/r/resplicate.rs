//! Hello-world snippet for `ProgrammingLanguageName::Resplicate`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#ResPlicate>
//! - <https://esolangs.org/wiki/ResPlicate>

use crate::data::programming_languages::ProgrammingParadigm::{QueueBased, Reflective};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Resplicate => "ResPlicate",
    versions: [Resplicate => "ResPlicate"],
    extensions: [],
}

const RESPLICATE: &str =
    r" 0 72 0 101 0 108 0 108 0 111 0 32 0 87 0 111 0 114 0 108 0 100 0 33 0 10";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Resplicate,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective, QueueBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Resplicate,
            syntax: RESPLICATE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RESPLICATE,
                    [(
                        TokenKind::Plain,
                        r" 0 72 0 101 0 108 0 108 0 111 0 32 0 87 0 111 0 114 0 108 0 100 0 33 0 10",
                    )],
                )
            },
        }],
    };
