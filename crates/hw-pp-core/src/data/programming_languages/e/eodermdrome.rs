//! Hello-world snippet for `ProgrammingLanguageName::Eodermdrome`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Eodermdrome>
//! - <https://esolangs.org/wiki/Eodermdrome>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Eodermdrome => "Eodermdrome",
    versions: [Eodermdrome => "Eodermdrome"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Eodermdrome,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Eodermdrome,
            syntax: concat!(r"this (", output_for_hello_world!(), r") says... hi!"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"this (", output_for_hello_world!(), r") says... hi!"),
            )],
        }],
    };
