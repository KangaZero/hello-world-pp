//! Hello-world snippet for `ProgrammingLanguageName::Sensetalk`.
//!
//! Imported from the Hello World Collection (`files/SenseTalk.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SenseTalk.txt>
//! - <http://helloworldcollection.de/#SenseTalk>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Sensetalk => "SenseTalk",
    versions: [Sensetalk => "SenseTalk"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sensetalk,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sensetalk,
            syntax: concat!(
                output_for_hello_world!(),
                r#" in SenseTalk.

on run put ""#,
                output_for_hello_world!(),
                r#"" end run "#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    output_for_hello_world!(),
                    r#" in SenseTalk.

on run put ""#,
                    output_for_hello_world!(),
                    r#"" end run "#
                ),
            )],
        }],
    };
