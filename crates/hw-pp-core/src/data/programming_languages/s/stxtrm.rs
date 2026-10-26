//! Hello-world snippet for `ProgrammingLanguageName::Stxtrm`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#STXTRM>
//! - <https://esolangs.org/wiki/STXTRM>

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
    language: Stxtrm => "STXTRM",
    versions: [Stxtrm => "STXTRM"],
    extensions: [],
}

const STXTRM: &str = concat!(r"[", output_for_hello_world!(), r"]");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stxtrm,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective, StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stxtrm,
            syntax: STXTRM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    STXTRM,
                    [(
                        TokenKind::Plain,
                        concat!(r"[", output_for_hello_world!(), r"]"),
                    )],
                )
            },
        }],
    };
