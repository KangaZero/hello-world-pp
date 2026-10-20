//! Hello-world snippet for `ProgrammingLanguageName::BangBangBrainfeed`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#!!brainfeed>
//! - <https://esolangs.org/wiki/!!brainfeed>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: BangBangBrainfeed => "!!brainfeed",
    versions: [BangBangBrainfeed => "!!brainfeed"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BangBangBrainfeed,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BangBangBrainfeed,
            syntax: r"+++++++?---,+++++++,,+++,#++++!--
!+++++++++++
+++++++++
?--------,
+++,------,#+++,#!",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"+++++++?---,+++++++,,+++,#++++!--
!+++++++++++
+++++++++
?--------,
+++,------,#+++,#!",
            )],
        }],
    };
