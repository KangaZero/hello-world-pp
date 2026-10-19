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
    programming_languages_token::TokenKind,
};

language_names! {
    language: RemoveLineNumbers => "Remove Line Numbers",
    versions: [RemoveLineNumbers => "Remove Line Numbers"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::RemoveLineNumbers,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::RemoveLineNumbers,
            syntax: concat!(r"1 ", output_for_hello_world!()),
            tokenized_syntax: &[(TokenKind::Plain, concat!(r"1 ", output_for_hello_world!()))],
        }],
    };
