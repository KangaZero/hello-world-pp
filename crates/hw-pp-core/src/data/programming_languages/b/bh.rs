//! Hello-world snippet for `ProgrammingLanguageName::BH`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#B-H>
//! - <https://esolangs.org/wiki/B-H>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: BH => "B-H",
    versions: [BH => "B-H"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BH,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BH,
            syntax: r"++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++",
            )],
        }],
    };
