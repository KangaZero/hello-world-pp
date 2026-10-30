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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Gynkosoft => "GynkoSoft",
    versions: [Gynkosoft => "GynkoSoft"],
    extensions: [],
}

const GYNKOSOFT: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gynkosoft,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gynkosoft,
            syntax: GYNKOSOFT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GYNKOSOFT,
                    [(
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
                )
            },
        }],
    };
