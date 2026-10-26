//! Hello-world snippet for `ProgrammingLanguageName::Aubergine`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#Aubergine>
//! - <https://esolangs.org/wiki/Aubergine>

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Reflective};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Aubergine => "Aubergine",
    versions: [Aubergine => "Aubergine"],
    extensions: [],
}

const AUBERGINE: &str = r"=aA-a1=oA=bi+b1-Ab-bb:bA+B1=iBGolf by Quintopia
!dlroW ,olleH";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aubergine,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, Reflective],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aubergine,
            syntax: AUBERGINE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AUBERGINE,
                    [(
                        TokenKind::Plain,
                        r"=aA-a1=oA=bi+b1-Ab-bb:bA+B1=iBGolf by Quintopia
!dlroW ,olleH",
                    )],
                )
            },
        }],
    };
