//! Hello-world snippet for `ProgrammingLanguageName::BrainfuckExtended`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Brainfuck_extended>
//! - <https://esolangs.org/wiki/Brainfuck_extended>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: BrainfuckExtended => "Brainfuck extended",
    versions: [BrainfuckExtended => "Brainfuck extended"],
    extensions: [],
}

const BRAINFUCK_EXTENDED: &str = concat!(r"$", output_for_hello_world!(), r"$[.>]");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BrainfuckExtended,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BrainfuckExtended,
            syntax: BRAINFUCK_EXTENDED,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BRAINFUCK_EXTENDED,
                    [(
                        TokenKind::Plain,
                        concat!(r"$", output_for_hello_world!(), r"$[.>]"),
                    )],
                )
            },
        }],
    };
