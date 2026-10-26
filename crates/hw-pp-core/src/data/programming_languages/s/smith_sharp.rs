//! Hello-world snippet for `ProgrammingLanguageName::SmithSharp`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#SMITH%23>
//! - <https://esolangs.org/wiki/SMITH_sharp>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: SmithSharp => "SMITH#",
    versions: [SmithSharp => "SMITH#"],
    extensions: [],
}

const SMITH_SHARP: &str = concat!(
    r"; SMITH# ",
    output_for_hello_world!(),
    r" program written by zzo38
; This code uses 33 cells. The number 8 command takes the cell
; number as the parameter and output it. Number 0 command stops
; the program. The first cell is cell zero. Even since cell 32 is
; referenced in the data section, I can reference the reference to cell
; 32 for space rather than adding the space into the data section.
8 25 8 26 8 27 8 27 8 28 8 23 8 29 8 28 8 30 8 27 8 31 8 32 0 72 101 108 111 87 114 100 33"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SmithSharp,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SmithSharp,
            syntax: SMITH_SHARP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SMITH_SHARP,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"; SMITH# ",
                            output_for_hello_world!(),
                            r" program written by zzo38
; This code uses 33 cells. The number 8 command takes the cell
; number as the parameter and output it. Number 0 command stops
; the program. The first cell is cell zero. Even since cell 32 is
; referenced in the data section, I can reference the reference to cell
; 32 for space rather than adding the space into the data section.
8 25 8 26 8 27 8 27 8 28 8 23 8 29 8 28 8 30 8 27 8 31 8 32 0 72 101 108 111 87 114 100 33"
                        ),
                    )],
                )
            },
        }],
    };
