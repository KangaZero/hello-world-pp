//! Hello-world snippet for `ProgrammingLanguageName::Muriel`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Muriel>
//! - <https://esolangs.org/wiki/Muriel>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Muriel => "Muriel",
    versions: [Muriel => "Muriel"],
    extensions: [],
}

const MURIEL: &str = concat!(r#".""#, output_for_hello_world!(), r#"";"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Muriel,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Muriel,
            syntax: MURIEL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MURIEL,
                    [(
                        TokenKind::Plain,
                        concat!(r#".""#, output_for_hello_world!(), r#"";"#),
                    )],
                )
            },
        }],
    };
