//! Hello-world snippet for `ProgrammingLanguageName::Expressions`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Expressions>
//! - <https://esolangs.org/wiki/Expressions>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Expressions => "Expressions",
    versions: [Expressions => "Expressions"],
    extensions: [],
}

const EXPRESSIONS: &str = concat!(r#"#0 print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Expressions,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Expressions,
            syntax: EXPRESSIONS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EXPRESSIONS,
                    [(
                        TokenKind::Plain,
                        concat!(r#"#0 print(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
