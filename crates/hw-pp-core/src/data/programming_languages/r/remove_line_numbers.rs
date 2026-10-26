//! Hello-world snippet for `ProgrammingLanguageName::RemoveLineNumbers`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Remove_Line_Numbers>
//! - <https://esolangs.org/wiki/Remove_Line_Numbers>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: RemoveLineNumbers => "Remove Line Numbers",
    versions: [RemoveLineNumbers => "Remove Line Numbers"],
    extensions: [],
}

const REMOVE_LINE_NUMBERS: &str = concat!(r"1 ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::RemoveLineNumbers,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::RemoveLineNumbers,
            syntax: REMOVE_LINE_NUMBERS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    REMOVE_LINE_NUMBERS,
                    [(TokenKind::Plain, concat!(r"1 ", output_for_hello_world!()))],
                )
            },
        }],
    };
