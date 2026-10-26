//! Hello-world snippet for `ProgrammingLanguageName::Timers`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Timers>
//! - <https://esolangs.org/wiki/Timers>

use crate::data::programming_languages::ProgrammingParadigm::{Reflective, StackBased};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Timers => "Timers",
    versions: [Timers => "Timers"],
    extensions: [],
}

const TIMERS: &str = concat!(r"(['", output_for_hello_world!(), r"\n']~)-(^,~)");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Timers,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective, StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Timers,
            syntax: TIMERS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TIMERS,
                    [(
                        TokenKind::Plain,
                        concat!(r"(['", output_for_hello_world!(), r"\n']~)-(^,~)"),
                    )],
                )
            },
        }],
    };
