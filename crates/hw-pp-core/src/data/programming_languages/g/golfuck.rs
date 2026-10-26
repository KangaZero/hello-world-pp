//! Hello-world snippet for `ProgrammingLanguageName::Golfuck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#golfuck>
//! - <https://esolangs.org/wiki/Golfuck>

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Golfuck => "golfuck",
    versions: [Golfuck => "golfuck"],
    extensions: [],
}

const GOLFUCK: &str = concat!(r"'", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Golfuck,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Golfuck,
            syntax: GOLFUCK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GOLFUCK,
                    [(TokenKind::Plain, concat!(r"'", output_for_hello_world!()))],
                )
            },
        }],
    };
