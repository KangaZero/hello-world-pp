//! Hello-world snippet for `ProgrammingLanguageName::Desrever`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Desrever>
//! - <https://esolangs.org/wiki/Desrever>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Desrever => "Desrever",
    versions: [Desrever => "Desrever"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Desrever,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Desrever,
            syntax: concat!(
                r#"{
  ;(""#,
                output_for_hello_world!(),
                r#"")print
} ()main func"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"{
  ;(""#,
                    output_for_hello_world!(),
                    r#"")print
} ()main func"#
                ),
            )],
        }],
    };
