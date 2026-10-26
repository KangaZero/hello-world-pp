//! Hello-world snippet for `ProgrammingLanguageName::Salt`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Salt>
//! - <https://esolangs.org/wiki/Salt>

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Salt => "Salt",
    versions: [Salt => "Salt"],
    extensions: [],
}

const SALT: &str = concat!(r"'", output_for_hello_world!(), r"'");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Salt,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Salt,
            syntax: SALT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SALT,
                    [(
                        TokenKind::Plain,
                        concat!(r"'", output_for_hello_world!(), r"'"),
                    )],
                )
            },
        }],
    };
