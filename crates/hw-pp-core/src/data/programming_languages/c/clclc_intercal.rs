//! Hello-world snippet for `ProgrammingLanguageName::ClclcIntercal`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#CLCLC-INTERCAL>
//! - <https://esolangs.org/wiki/CLCLC-INTERCAL>
//! - <https://esolangs.org/wiki/INTERCAL> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: ClclcIntercal => "CLCLC-INTERCAL",
    versions: [ClclcIntercal => "CLCLC-INTERCAL"],
    extensions: [],
}

const CLCLC_INTERCAL: &str = r"PLEASE ;1 <- #2
DO ;1 SUB #1 <- #17947$#20775
DO ;1 SUB #2 <- #5204$#21386
DO READ OUT ;1
PLEASE GIVE UP";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ClclcIntercal,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ClclcIntercal,
            syntax: CLCLC_INTERCAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CLCLC_INTERCAL,
                    [(
                        TokenKind::Plain,
                        r"PLEASE ;1 <- #2
DO ;1 SUB #1 <- #17947$#20775
DO ;1 SUB #2 <- #5204$#21386
DO READ OUT ;1
PLEASE GIVE UP",
                    )],
                )
            },
        }],
    };
