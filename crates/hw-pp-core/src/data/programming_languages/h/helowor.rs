//! Hello-world snippet for `ProgrammingLanguageName::Helowor`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#HELOWOR>
//! - <https://esolangs.org/wiki/HELOWOR>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Helowor => "HELOWOR",
    versions: [Helowor => "HELOWOR"],
    extensions: [],
}

const HELOWOR: &str = r"Hlo";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Helowor,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Helowor,
            syntax: HELOWOR,
            tokenized_syntax: &const { checked_token_lengths(HELOWOR, [(TokenKind::Plain, r"Hlo")]) },
        }],
    };
