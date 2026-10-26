//! Hello-world snippet for `ProgrammingLanguageName::Almfcplir`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#ALMFCPLIR>
//! - <https://esolangs.org/wiki/ALMFCPLIR>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Almfcplir => "ALMFCPLIR",
    versions: [Almfcplir => "ALMFCPLIR"],
    extensions: [],
}

const ALMFCPLIR: &str = concat!(
    r#"main > { print > {""#,
    output_for_hello_world!(),
    r#""} }"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Almfcplir,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Almfcplir,
            syntax: ALMFCPLIR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ALMFCPLIR,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"main > { print > {""#,
                            output_for_hello_world!(),
                            r#""} }"#
                        ),
                    )],
                )
            },
        }],
    };
