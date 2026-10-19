//! Hello-world snippet for `ProgrammingLanguageName::Gynkosoft`.
//!
//! Imported from the Hello World Collection (`files/GynkoSoft` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/GynkoSoft>
//! - <http://helloworldcollection.de/#GynkoSoft>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Gynkosoft => "GynkoSoft",
    versions: [Gynkosoft => "GynkoSoft"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gynkosoft,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gynkosoft,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r#" in GynkoSoft
; Simple version
0.00 Protocol ""#,
                output_for_hello_world!(),
                r#""


; "#,
                output_for_hello_world!(),
                r#" in GynkoSoft
; Dialog box output
0.00 Message ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"; ",
                    output_for_hello_world!(),
                    r#" in GynkoSoft
; Simple version
0.00 Protocol ""#,
                    output_for_hello_world!(),
                    r#""


; "#,
                    output_for_hello_world!(),
                    r#" in GynkoSoft
; Dialog box output
0.00 Message ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
