//! Hello-world snippet for `ProgrammingLanguageName::PythonButItReceivedTheWorstTranslation`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Python,_but_it_received_the_worst_translation.>
//! - <https://esolangs.org/wiki/Python,_but_it_received_the_worst_translation.>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: PythonButItReceivedTheWorstTranslation => "Python, but it received the worst translation.",
    versions: [PythonButItReceivedTheWorstTranslation => "Python, but it received the worst translation."],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PythonButItReceivedTheWorstTranslation,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PythonButItReceivedTheWorstTranslation,
            syntax: concat!(r#"version(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"version(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
