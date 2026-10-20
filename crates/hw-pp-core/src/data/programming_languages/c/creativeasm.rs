//! Hello-world snippet for `ProgrammingLanguageName::Creativeasm`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#CreativeASM>
//! - <https://esolangs.org/wiki/CreativeASM>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Creativeasm => "CreativeASM",
    versions: [Creativeasm => "CreativeASM"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Creativeasm,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Creativeasm,
            syntax: concat!(
                r#"put rax ""#,
                output_for_hello_world!(),
                r#""
int rax 01h"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"put rax ""#,
                    output_for_hello_world!(),
                    r#""
int rax 01h"#
                ),
            )],
        }],
    };
