//! Hello-world snippet for `ProgrammingLanguageName::BrianAndChuck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Brian_%26_Chuck>
//! - <https://esolangs.org/wiki/Brian_%26_Chuck>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: BrianAndChuck => "Brian & Chuck",
    versions: [BrianAndChuck => "Brian & Chuck"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BrianAndChuck,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BrianAndChuck,
            syntax: concat!(
                r"?",
                output_for_hello_world!(),
                r"
!>.>.>.>.>.>.>.>.>.>.>.>.>."
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"?",
                    output_for_hello_world!(),
                    r"
!>.>.>.>.>.>.>.>.>.>.>.>.>."
                ),
            )],
        }],
    };
