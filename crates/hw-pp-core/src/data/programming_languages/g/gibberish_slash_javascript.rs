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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: GibberishSlashJavascript => "Gibberish/JavaScript",
    versions: [GibberishSlashJavascript => "Gibberish/JavaScript"],
    extensions: [],
}

const GIBBERISH_SLASH_JAVASCRIPT: &str = concat!(r"${echo:", output_for_hello_world!(), r"}");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GibberishSlashJavascript,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GibberishSlashJavascript,
            syntax: GIBBERISH_SLASH_JAVASCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GIBBERISH_SLASH_JAVASCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(r"${echo:", output_for_hello_world!(), r"}"),
                    )],
                )
            },
        }],
    };
