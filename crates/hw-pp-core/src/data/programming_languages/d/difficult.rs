//! Hello-world snippet for `ProgrammingLanguageName::Difficult`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Difficult>
//! - <https://esolangs.org/wiki/Difficult>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Difficult => "Difficult",
    versions: [Difficult => "Difficult"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Difficult,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Difficult,
            syntax: concat!(
                r#"thing.console.print(""#,
                output_for_hello_world!(),
                r#"")?"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"thing.console.print(""#,
                    output_for_hello_world!(),
                    r#"")?"#
                ),
            )],
        }],
    };
