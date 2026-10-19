//! Hello-world snippet for `ProgrammingLanguageName::Splinter`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Splinter>
//! - <https://esolangs.org/wiki/Splinter>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Splinter => "Splinter",
    versions: [Splinter => "Splinter"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Splinter,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Splinter,
            syntax: r"\H\e\l\l\o\,\ \w\o\r\l\d\!\",
            tokenized_syntax: &[(TokenKind::Plain, r"\H\e\l\l\o\,\ \w\o\r\l\d\!\")],
        }],
    };
