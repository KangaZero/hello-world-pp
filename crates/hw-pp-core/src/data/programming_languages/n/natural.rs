//! Hello-world snippet for `ProgrammingLanguageName::Natural`.
//!
//! Imported from the Hello World Collection (`files/Natural.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Natural.txt>
//! - <http://helloworldcollection.de/#Natural>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Natural => "Natural",
    versions: [Natural => "Natural"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Natural,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Natural,
            syntax: concat!(
                r" * ",
                output_for_hello_world!(),
                r#" in Natural (by Software AG)
Write ""#,
                output_for_hello_world!(),
                r#""."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r" * ",
                    output_for_hello_world!(),
                    r#" in Natural (by Software AG)
Write ""#,
                    output_for_hello_world!(),
                    r#""."#
                ),
            )],
        }],
    };
