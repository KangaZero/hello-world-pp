//! Hello-world snippet for `ProgrammingLanguageName::ClclcIntercal`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#CLCLC-INTERCAL>
//! - <https://esolangs.org/wiki/CLCLC-INTERCAL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: ClclcIntercal => "CLCLC-INTERCAL",
    versions: [ClclcIntercal => "CLCLC-INTERCAL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ClclcIntercal,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ClclcIntercal,
            syntax: r"PLEASE ;1 <- #2
DO ;1 SUB #1 <- #17947$#20775
DO ;1 SUB #2 <- #5204$#21386
DO READ OUT ;1
PLEASE GIVE UP",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"PLEASE ;1 <- #2
DO ;1 SUB #1 <- #17947$#20775
DO ;1 SUB #2 <- #5204$#21386
DO READ OUT ;1
PLEASE GIVE UP",
            )],
        }],
    };
