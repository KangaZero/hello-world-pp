//! Hello-world snippet for `ProgrammingLanguageName::SelfModifyingBrainfuckSharp`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Self-modifying_Brainfuck%23>
//! - <https://esolangs.org/wiki/Self-modifying_Brainfuck_Sharp>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: SelfModifyingBrainfuckSharp => "Self-modifying Brainfuck#",
    versions: [SelfModifyingBrainfuckSharp => "Self-modifying Brainfuck#"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SelfModifyingBrainfuckSharp,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SelfModifyingBrainfuckSharp,
            syntax: concat!(
                output_for_hello_world!(),
                r"
----------[++++++++++.>----------]++++++++++"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    output_for_hello_world!(),
                    r"
----------[++++++++++.>----------]++++++++++"
                ),
            )],
        }],
    };
