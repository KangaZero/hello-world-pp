//! Hello-world snippet for `ProgrammingLanguageName::Setlang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Setlang>
//! - <https://esolangs.org/wiki/Setlang>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Setlang => "Setlang",
    versions: [Setlang => "Setlang"],
    extensions: [],
}

const SETLANG: &str = concat!(r#"(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Setlang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Setlang,
            syntax: SETLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SETLANG,
                    [(
                        TokenKind::Plain,
                        concat!(r#"(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
