//! Hello-world snippet for `ProgrammingLanguageName::Losescript`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Losescript>
//! - <https://esolangs.org/wiki/Losescript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Losescript => "Losescript",
    versions: [Losescript => "Losescript"],
    extensions: [],
}

const LOSESCRIPT: &str = r"H";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Losescript,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Losescript,
            syntax: LOSESCRIPT,
            tokenized_syntax: &const { checked_token_lengths(LOSESCRIPT, [(TokenKind::Plain, r"H")]) },
        }],
    };
