//! Hello-world snippet for `ProgrammingLanguageName::Bird`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Bird>
//! - <https://esolangs.org/wiki/Bird>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Bird => "Bird",
    versions: [Bird => "Bird"],
    extensions: [],
}

const BIRD: &str = concat!(r"'", output_for_hello_world!(), r"'/~");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bird,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bird,
            syntax: BIRD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BIRD,
                    [(
                        TokenKind::Plain,
                        concat!(r"'", output_for_hello_world!(), r"'/~"),
                    )],
                )
            },
        }],
    };
