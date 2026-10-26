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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: SelfModifyingBrainfuckSharp => "Self-modifying Brainfuck#",
    versions: [SelfModifyingBrainfuckSharp => "Self-modifying Brainfuck#"],
    extensions: [],
}

const SELF_MODIFYING_BRAINFUCK_SHARP: &str = concat!(
    output_for_hello_world!(),
    r"
----------[++++++++++.>----------]++++++++++"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SelfModifyingBrainfuckSharp,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SelfModifyingBrainfuckSharp,
            syntax: SELF_MODIFYING_BRAINFUCK_SHARP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SELF_MODIFYING_BRAINFUCK_SHARP,
                    [(
                        TokenKind::Plain,
                        concat!(
                            output_for_hello_world!(),
                            r"
----------[++++++++++.>----------]++++++++++"
                        ),
                    )],
                )
            },
        }],
    };
