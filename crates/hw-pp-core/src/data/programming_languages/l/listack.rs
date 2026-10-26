//! Hello-world snippet for `ProgrammingLanguageName::Listack`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Listack>
//! - <https://esolangs.org/wiki/Listack>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Listack => "Listack",
    versions: [Listack => "Listack"],
    extensions: [],
}

const LISTACK: &str = concat!(r#" ""#, output_for_hello_world!(), r#"" .println"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Listack,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Listack,
            syntax: LISTACK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LISTACK,
                    [(
                        TokenKind::Plain,
                        concat!(r#" ""#, output_for_hello_world!(), r#"" .println"#),
                    )],
                )
            },
        }],
    };
