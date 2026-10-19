//! Hello-world snippet for `ProgrammingLanguageName::Erre`.
//!
//! Imported from the Hello World Collection (`files/ERRE.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ERRE.htm>
//! - <http://helloworldcollection.de/#ERRE>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Erre => "ERRE",
    versions: [Erre => "ERRE"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Erre,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Erre,
            syntax: concat!(
                r"! ",
                output_for_hello_world!(),
                r#" in ERRE

PROGRAM HELLO
BEGIN
  PRINT(""#,
                output_for_hello_world!(),
                r#"")
END PROGRAM"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"! ",
                    output_for_hello_world!(),
                    r#" in ERRE

PROGRAM HELLO
BEGIN
  PRINT(""#,
                    output_for_hello_world!(),
                    r#"")
END PROGRAM"#
                ),
            )],
        }],
    };
