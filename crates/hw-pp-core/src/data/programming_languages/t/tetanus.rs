//! Hello-world snippet for `ProgrammingLanguageName::Tetanus`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Tetanus>
//! - <https://esolangs.org/wiki/Tetanus>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Tetanus => "Tetanus",
    versions: [Tetanus => "Tetanus"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tetanus,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tetanus,
            syntax: concat!(
                r"^(H.+)
!\1~
",
                output_for_hello_world!()
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"^(H.+)
!\1~
",
                    output_for_hello_world!()
                ),
            )],
        }],
    };
