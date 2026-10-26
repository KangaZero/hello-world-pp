//! Hello-world snippet for `ProgrammingLanguageName::Sinclairbasic`.
//!
//! Imported from the Hello World Collection (`files/SinclairBasic.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SinclairBasic.txt>
//! - <http://helloworldcollection.de/#SinclairBasic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sinclairbasic => "SinclairBasic",
    versions: [Sinclairbasic => "SinclairBasic"],
    extensions: [],
}

const SINCLAIRBASIC: &str = concat!(
    r"10 REM ",
    output_for_hello_world!(),
    r#" in Sinclair BASIC
20 PRINT ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sinclairbasic,
        extensions: &[],
        categories: &[],
        families: &[Basic],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sinclairbasic,
            syntax: SINCLAIRBASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SINCLAIRBASIC,
                    [
                        (TokenKind::Number, r"10"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"REM"),
                        (
                            TokenKind::Plain,
                            concat!(
                                r" ",
                                output_for_hello_world!(),
                                r" in Sinclair BASIC
"
                            ),
                        ),
                        (TokenKind::Number, r"20"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"PRINT"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
