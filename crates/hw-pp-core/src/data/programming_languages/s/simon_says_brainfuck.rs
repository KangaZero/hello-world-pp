//! Hello-world snippet for `ProgrammingLanguageName::SimonSaysBrainfuck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Simon_says_brainfuck>
//! - <https://esolangs.org/wiki/Simon_says_brainfuck>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: SimonSaysBrainfuck => "Simon says brainfuck",
    versions: [SimonSaysBrainfuck => "Simon says brainfuck"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SimonSaysBrainfuck,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SimonSaysBrainfuck,
            syntax: r"Simon says: +++++++++[>++++++++<-]>.>++++++++++
Simon says: [>++++++++++<-]>+.+++++++..+++.>++++
Simon says: [>++++++++<-]>.<<<<+++++++++++++++.>>.+++.------.--------.",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"Simon says: +++++++++[>++++++++<-]>.>++++++++++
Simon says: [>++++++++++<-]>+.+++++++..+++.>++++
Simon says: [>++++++++<-]>.<<<<+++++++++++++++.>>.+++.------.--------.",
            )],
        }],
    };
