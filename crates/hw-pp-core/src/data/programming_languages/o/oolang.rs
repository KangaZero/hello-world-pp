//! Hello-world snippet for `ProgrammingLanguageName::Oolang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#OOLANG>
//! - <https://esolangs.org/wiki/OOLANG>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Oolang => "OOLANG",
    versions: [Oolang => "OOLANG"],
    extensions: [],
}

const OOLANG: &str = concat!(output_for_hello_world!(), r".println;");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Oolang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Oolang,
            syntax: OOLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OOLANG,
                    [(
                        TokenKind::Plain,
                        concat!(output_for_hello_world!(), r".println;"),
                    )],
                )
            },
        }],
    };
