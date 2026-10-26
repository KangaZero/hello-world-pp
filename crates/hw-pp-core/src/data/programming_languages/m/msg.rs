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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Msg => "MSG",
    versions: [Msg => "MSG"],
    extensions: [],
}

const MSG: &str = concat!(r"main 'passon stdout '", output_for_hello_world!(), r"' '");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Msg,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Msg,
            syntax: MSG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MSG,
                    [(
                        TokenKind::Plain,
                        concat!(r"main 'passon stdout '", output_for_hello_world!(), r"' '"),
                    )],
                )
            },
        }],
    };
