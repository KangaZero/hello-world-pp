//! Hello-world snippet for `ProgrammingLanguageName::Powerbasic`.
//!
//! Imported from the Hello World Collection (`files/Powerbasic.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Powerbasic.txt>
//! - <http://helloworldcollection.de/#Powerbasic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Powerbasic => "Powerbasic",
    versions: [Powerbasic => "Powerbasic"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Powerbasic,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Powerbasic,
            syntax: concat!(
                r"' ",
                output_for_hello_world!(),
                r#" in Powerbasic Console Compiler

FUNCTION PBMAIN () AS LONG

    PRINT ""#,
                output_for_hello_world!(),
                r#""

END FUNCTION"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"' ",
                    output_for_hello_world!(),
                    r#" in Powerbasic Console Compiler

FUNCTION PBMAIN () AS LONG

    PRINT ""#,
                    output_for_hello_world!(),
                    r#""

END FUNCTION"#
                ),
            )],
        }],
    };
