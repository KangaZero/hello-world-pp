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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Creativeasm => "CreativeASM",
    versions: [Creativeasm => "CreativeASM"],
    extensions: [],
}

const CREATIVEASM: &str = concat!(
    r#"put rax ""#,
    output_for_hello_world!(),
    r#""
int rax 01h"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Creativeasm,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Creativeasm,
            syntax: CREATIVEASM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CREATIVEASM,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"put rax ""#,
                            output_for_hello_world!(),
                            r#""
int rax 01h"#
                        ),
                    )],
                )
            },
        }],
    };
