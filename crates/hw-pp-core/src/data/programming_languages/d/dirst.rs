//! Hello-world snippet for `ProgrammingLanguageName::Dirst`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Dirst>
//! - <https://esolangs.org/wiki/Dirst>

use crate::data::programming_languages::ProgrammingParadigm::Structured;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Dirst => "Dirst",
    versions: [Dirst => "Dirst"],
    extensions: [],
}

const DIRST: &str = concat!(r"dss_", output_for_hello_world!(), r"-e.txt");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dirst,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Structured],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dirst,
            syntax: DIRST,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DIRST,
                    [(
                        TokenKind::Plain,
                        concat!(r"dss_", output_for_hello_world!(), r"-e.txt"),
                    )],
                )
            },
        }],
    };
