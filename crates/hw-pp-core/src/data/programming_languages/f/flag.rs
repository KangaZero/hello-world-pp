//! Hello-world snippet for `ProgrammingLanguageName::Flag`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#flag>
//! - <https://esolangs.org/wiki/Flag>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Flag => "flag",
    versions: [Flag => "flag"],
    extensions: [],
}

const FLAG: &str = concat!(output_for_hello_world!(), r"_!");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Flag,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Flag,
            syntax: FLAG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FLAG,
                    [(TokenKind::Plain, concat!(output_for_hello_world!(), r"_!"))],
                )
            },
        }],
    };
