//! Hello-world snippet for `ProgrammingLanguageName::Modulous`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Modulous>
//! - <https://esolangs.org/wiki/Modulous>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Modulous => "Modulous",
    versions: [Modulous => "Modulous"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Modulous,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Modulous,
            syntax: concat!(
                r"[PSH STR “",
                output_for_hello_world!(),
                r"”][PRT STR][JMP B 1 IF NOT 0]"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"[PSH STR “",
                    output_for_hello_world!(),
                    r"”][PRT STR][JMP B 1 IF NOT 0]"
                ),
            )],
        }],
    };
