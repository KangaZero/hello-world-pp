//! Hello-world snippet for `ProgrammingLanguageName::Msg`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#MSG>
//! - <https://esolangs.org/wiki/MSG>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Msg => "MSG",
    versions: [Msg => "MSG"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Msg,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Msg,
            syntax: concat!(r"main 'passon stdout '", output_for_hello_world!(), r"' '"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"main 'passon stdout '", output_for_hello_world!(), r"' '"),
            )],
        }],
    };
