//! Hello-world snippet for `ProgrammingLanguageName::Dl`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#DL>
//! - <https://esolangs.org/wiki/DL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dl => "DL",
    versions: [Dl => "DL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dl,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dl,
            syntax: concat!(
                r"DL Start
say(",
                output_for_hello_world!(),
                r")
DL End"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"DL Start
say(",
                    output_for_hello_world!(),
                    r")
DL End"
                ),
            )],
        }],
    };
