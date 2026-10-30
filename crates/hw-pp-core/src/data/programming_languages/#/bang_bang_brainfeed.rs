//! Hello-world snippet for `ProgrammingLanguageName::BangBangBrainfeed`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#!!brainfeed>
//! - <https://esolangs.org/wiki/!!brainfeed>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: BangBangBrainfeed => "!!brainfeed",
    versions: [BangBangBrainfeed => "!!brainfeed"],
    extensions: [],
}

const BANG_BANG_BRAINFEED: &str = r"+++++++?---,+++++++,,+++,#++++!--
!+++++++++++
+++++++++
?--------,
+++,------,#+++,#!";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BangBangBrainfeed,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BangBangBrainfeed,
            syntax: BANG_BANG_BRAINFEED,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BANG_BANG_BRAINFEED,
                    [(
                        TokenKind::Plain,
                        r"+++++++?---,+++++++,,+++,#++++!--
!+++++++++++
+++++++++
?--------,
+++,------,#+++,#!",
                    )],
                )
            },
        }],
    };
