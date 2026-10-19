//! Hello-world snippet for `ProgrammingLanguageName::Satire`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#SATire>
//! - <https://esolangs.org/wiki/SATire>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Satire => "SATire",
    versions: [Satire => "SATire"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Satire,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Satire,
            syntax: concat!(
                r#"Please fill out the following form.
greeting: ""#,
                output_for_hello_world!(),
                r#"\0A"
Calculator section.

(1) Evaluate 1. Round up.
a. ([] # greeting) # 1
b. ["",2,]
c. ["",3,]
d. ["",None-of-the-above,]"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"Please fill out the following form.
greeting: ""#,
                    output_for_hello_world!(),
                    r#"\0A"
Calculator section.

(1) Evaluate 1. Round up.
a. ([] # greeting) # 1
b. ["",2,]
c. ["",3,]
d. ["",None-of-the-above,]"#
                ),
            )],
        }],
    };
