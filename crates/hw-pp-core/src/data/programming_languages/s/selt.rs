//! Hello-world snippet for `ProgrammingLanguageName::Selt`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Selt>
//! - <https://esolangs.org/wiki/Selt>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Selt => "Selt",
    versions: [Selt => "Selt"],
    extensions: [],
}

const SELT: &str = r"println Hello,\ World\!";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Selt,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Selt,
            syntax: SELT,
            tokenized_syntax: &const {
                checked_token_lengths(SELT, [(TokenKind::Plain, r"println Hello,\ World\!")])
            },
        }],
    };
