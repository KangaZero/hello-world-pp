//! Hello-world snippet for `ProgrammingLanguageName::Hexcellent`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Hexcellent>
//! - <https://esolangs.org/wiki/Hexcellent>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Hexcellent => "Hexcellent",
    versions: [Hexcellent => "Hexcellent"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hexcellent,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hexcellent,
            syntax: concat!(
                r"HXCL��
þí�`",
                output_for_hello_world!(),
                r"`�"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"HXCL��
þí�`",
                    output_for_hello_world!(),
                    r"`�"
                ),
            )],
        }],
    };
