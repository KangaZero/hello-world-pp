//! Hello-world snippet for `ProgrammingLanguageName::Purple`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Purple>
//! - <https://esolangs.org/wiki/Purple>

use crate::data::programming_languages::ProgrammingParadigm::{CellBased, Imperative, Reflective};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Purple => "Purple",
    versions: [Purple => "Purple"],
    extensions: [],
}

const PURPLE: &str = r"aA1aa1oA0bi0b0bb1bbbAiB0 Purple is the awesomest, so why haven't you tried it yet?
!dlroW ,olleH";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Purple,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, Reflective, CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Purple,
            syntax: PURPLE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PURPLE,
                    [(
                        TokenKind::Plain,
                        r"aA1aa1oA0bi0b0bb1bbbAiB0 Purple is the awesomest, so why haven't you tried it yet?
!dlroW ,olleH",
                    )],
                )
            },
        }],
    };
