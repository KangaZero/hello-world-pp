//! Hello-world snippet for `ProgrammingLanguageName::Bcdfuck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#BCDFuck>
//! - <https://esolangs.org/wiki/BCDFuck>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Bcdfuck => "BCDFuck",
    versions: [Bcdfuck => "BCDFuck"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bcdfuck,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bcdfuck,
            syntax: r"4A E4 AE 7A 7A E7 A8 AE 1A E6 AE 5A 5A E6 AE 5A 5A E4 A5 AE 3A E0 EA 3A
E2 AE 7A 8A E4 A5 AE 8A E5 A6 AE 4A E6 AE 5A 5A E7 A7 AE 7A 7A E7 A8 AE
0E A4 A5 AE",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"4A E4 AE 7A 7A E7 A8 AE 1A E6 AE 5A 5A E6 AE 5A 5A E4 A5 AE 3A E0 EA 3A
E2 AE 7A 8A E4 A5 AE 8A E5 A6 AE 4A E6 AE 5A 5A E7 A7 AE 7A 7A E7 A8 AE
0E A4 A5 AE",
            )],
        }],
    };
