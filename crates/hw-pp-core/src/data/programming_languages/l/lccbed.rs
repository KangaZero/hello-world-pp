//! Hello-world snippet for `ProgrammingLanguageName::Lccbed`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#LCCBED>
//! - <https://esolangs.org/wiki/LCCBED>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Lccbed => "LCCBED",
    versions: [Lccbed => "LCCBED"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lccbed,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lccbed,
            syntax: r"p(72)ofp(101)ofp(108)ofp(108)ofp(111)ofp(44)ofp(32)ofp(87)ofp(111)ofp(114)ofp(108)ofp(100)ofp(33)o",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"p(72)ofp(101)ofp(108)ofp(108)ofp(111)ofp(44)ofp(32)ofp(87)ofp(111)ofp(114)ofp(108)ofp(100)ofp(33)o",
            )],
        }],
    };
