//! Hello-world snippet for `ProgrammingLanguageName::Sclipting`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Sclipting>
//! - <https://esolangs.org/wiki/Sclipting>

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sclipting => "Sclipting",
    versions: [Sclipting => "Sclipting"],
    extensions: [],
}

const SCLIPTING: &str = r"丟낆녬닆묬긅덯댦롤긐뤊";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sclipting,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sclipting,
            syntax: SCLIPTING,
            tokenized_syntax: &const {
                checked_token_lengths(SCLIPTING, [(TokenKind::Plain, r"丟낆녬닆묬긅덯댦롤긐뤊")])
            },
        }],
    };
