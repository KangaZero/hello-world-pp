//! Hello-world snippet for `ProgrammingLanguageName::Ero`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Ero>
//! - <https://esolangs.org/wiki/Ero>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ero => "Ero",
    versions: [Ero => "Ero"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ero,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ero,
            syntax: concat!(
                r#":.1|::1>>""#,
                output_for_hello_world!(),
                r#"":|:
:.8|::2>>1>>1:|:"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#":.1|::1>>""#,
                    output_for_hello_world!(),
                    r#"":|:
:.8|::2>>1>>1:|:"#
                ),
            )],
        }],
    };
