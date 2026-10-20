//! Hello-world snippet for `ProgrammingLanguageName::GibberishSlashJavascript`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Gibberish%2FJavaScript>
//! - <https://esolangs.org/wiki/Gibberish%2FJavaScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: GibberishSlashJavascript => "Gibberish/JavaScript",
    versions: [GibberishSlashJavascript => "Gibberish/JavaScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GibberishSlashJavascript,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GibberishSlashJavascript,
            syntax: concat!(r"${echo:", output_for_hello_world!(), r"}"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"${echo:", output_for_hello_world!(), r"}"),
            )],
        }],
    };
