//! Hello-world snippet for `ProgrammingLanguageName::Rhotor`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Rhotor>
//! - <https://esolangs.org/wiki/Rhotor>

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rhotor => "Rhotor",
    versions: [Rhotor => "Rhotor"],
    extensions: [],
}

const RHOTOR: &str = concat!(r#"x/%""#, output_for_hello_world!(), r#"\n""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rhotor,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rhotor,
            syntax: RHOTOR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RHOTOR,
                    [(
                        TokenKind::Plain,
                        concat!(r#"x/%""#, output_for_hello_world!(), r#"\n""#),
                    )],
                )
            },
        }],
    };
