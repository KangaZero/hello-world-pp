//! Hello-world snippet for `ProgrammingLanguageName::Sinclairbasic`.
//!
//! Imported from the Hello World Collection (`files/SinclairBasic.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SinclairBasic.txt>
//! - <http://helloworldcollection.de/#SinclairBasic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Sinclairbasic => "SinclairBasic",
    versions: [Sinclairbasic => "SinclairBasic"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sinclairbasic,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sinclairbasic,
            syntax: concat!(
                r"10 REM ",
                output_for_hello_world!(),
                r#" in Sinclair BASIC
20 PRINT ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"10 REM ",
                    output_for_hello_world!(),
                    r#" in Sinclair BASIC
20 PRINT ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
