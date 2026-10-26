//! Hello-world snippet for `ProgrammingLanguageName::Empty`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Empty>
//! - <https://esolangs.org/wiki/Empty>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Empty => "Empty",
    versions: [Empty => "Empty"],
    extensions: [],
}

const EMPTY: &str = r#"Hello," World!:"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Empty,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Empty,
            syntax: EMPTY,
            tokenized_syntax: &const {
                checked_token_lengths(EMPTY, [(TokenKind::Plain, r#"Hello," World!:"#)])
            },
        }],
    };
