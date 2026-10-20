//! Hello-world snippet for `ProgrammingLanguageName::Rhotor`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Rhotor>
//! - <https://esolangs.org/wiki/Rhotor>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Rhotor => "Rhotor",
    versions: [Rhotor => "Rhotor"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rhotor,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rhotor,
            syntax: concat!(r#"x/%""#, output_for_hello_world!(), r#"\n""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"x/%""#, output_for_hello_world!(), r#"\n""#),
            )],
        }],
    };
