//! Hello-world snippet for `ProgrammingLanguageName::Commercial`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Commercial>
//! - <https://esolangs.org/wiki/Commercial>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Commercial => "Commercial",
    versions: [Commercial => "Commercial"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Commercial,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Commercial,
            syntax: concat!(
                r#"""#,
                output_for_hello_world!(),
                r#"" - Satisfied Consumer of Hello
Hello has been selling out worldwide!"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"""#,
                    output_for_hello_world!(),
                    r#"" - Satisfied Consumer of Hello
Hello has been selling out worldwide!"#
                ),
            )],
        }],
    };
