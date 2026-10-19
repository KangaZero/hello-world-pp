//! Hello-world snippet for `ProgrammingLanguageName::Continuousequation`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#ContinuousEquation>
//! - <https://esolangs.org/wiki/ContinuousEquation>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Continuousequation => "ContinuousEquation",
    versions: [Continuousequation => "ContinuousEquation"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Continuousequation,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Continuousequation,
            syntax: r"0 0 o'H'e'l'l'o',' 'w'o'r'l'd'!",
            tokenized_syntax: &[(TokenKind::Plain, r"0 0 o'H'e'l'l'o',' 'w'o'r'l'd'!")],
        }],
    };
