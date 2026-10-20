//! Hello-world snippet for `ProgrammingLanguageName::Tiny`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Tiny>
//! - <https://esolangs.org/wiki/Tiny>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Tiny => "Tiny",
    versions: [Tiny => "Tiny"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tiny,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tiny,
            syntax: concat!(
                r#"1 ""#,
                output_for_hello_world!(),
                r#"\n"
2 ."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"1 ""#,
                    output_for_hello_world!(),
                    r#"\n"
2 ."#
                ),
            )],
        }],
    };
