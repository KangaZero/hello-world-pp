//! Hello-world snippet for `ProgrammingLanguageName::Choon`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Choon>
//! - <https://esolangs.org/wiki/Choon>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Choon => "Choon",
    versions: [Choon => "Choon"],
    extensions: [],
}

const CHOON: &str = r"AGb-A#A#+A+%A#DF-AC#";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Choon,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Choon,
            syntax: CHOON,
            tokenized_syntax: &const {
                checked_token_lengths(CHOON, [(TokenKind::Plain, r"AGb-A#A#+A+%A#DF-AC#")])
            },
        }],
    };
