//! Hello-world snippet for `ProgrammingLanguageName::TurthMachine`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Turth-machine>
//! - <https://esolangs.org/wiki/Turth-machine>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: TurthMachine => "Turth-machine",
    versions: [TurthMachine => "Turth-machine"],
    extensions: [],
}

const TURTH_MACHINE: &str = concat!(r#"Output ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TurthMachine,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TurthMachine,
            syntax: TURTH_MACHINE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TURTH_MACHINE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"Output ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
