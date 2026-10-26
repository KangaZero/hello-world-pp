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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Hexcellent => "Hexcellent",
    versions: [Hexcellent => "Hexcellent"],
    extensions: [],
}

const HEXCELLENT: &str = concat!(
    r"HXCL��
þí�`",
    output_for_hello_world!(),
    r"`�"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hexcellent,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hexcellent,
            syntax: HEXCELLENT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HEXCELLENT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"HXCL��
þí�`",
                            output_for_hello_world!(),
                            r"`�"
                        ),
                    )],
                )
            },
        }],
    };
