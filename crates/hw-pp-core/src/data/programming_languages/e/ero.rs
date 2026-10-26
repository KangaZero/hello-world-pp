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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ero => "Ero",
    versions: [Ero => "Ero"],
    extensions: [],
}

const ERO: &str = concat!(
    r#":.1|::1>>""#,
    output_for_hello_world!(),
    r#"":|:
:.8|::2>>1>>1:|:"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ero,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ero,
            syntax: ERO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ERO,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#":.1|::1>>""#,
                            output_for_hello_world!(),
                            r#"":|:
:.8|::2>>1>>1:|:"#
                        ),
                    )],
                )
            },
        }],
    };
