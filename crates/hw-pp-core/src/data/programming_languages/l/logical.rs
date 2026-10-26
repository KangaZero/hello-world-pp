//! Hello-world snippet for `ProgrammingLanguageName::Logical`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Logical>
//! - <https://esolangs.org/wiki/Logical>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Logical => "Logical",
    versions: [Logical => "Logical"],
    extensions: [],
}

const LOGICAL: &str = r"COMET DEFN !DLROW_,OLLEH";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Logical,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Logical,
            syntax: LOGICAL,
            tokenized_syntax: &const {
                checked_token_lengths(LOGICAL, [(TokenKind::Plain, r"COMET DEFN !DLROW_,OLLEH")])
            },
        }],
    };
