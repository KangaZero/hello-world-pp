//! Hello-world snippet for `ProgrammingLanguageName::VbaExcel`.
//!
//! Imported from the Hello World Collection (`files/VBA&nbsp;(Excel).htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/VBA%26nbsp%3B%28Excel%29.htm>
//! - <http://helloworldcollection.de/#VBA%C2%A0(Excel)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: VbaExcel => "VBA (Excel)",
    versions: [VbaExcel => "VBA (Excel)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VbaExcel,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VbaExcel,
            syntax: concat!(
                r"' ",
                output_for_hello_world!(),
                r#" in Visual Basic for Applications, Excel version

Private Sub Workbook_Open()
    MsgBox ""#,
                output_for_hello_world!(),
                r#""
End Sub"#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    concat!(
                        r"' ",
                        output_for_hello_world!(),
                        r" in Visual Basic for Applications, Excel version"
                    ),
                ),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"Private"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"Sub"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"Workbook_Open"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"MsgBox"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"End"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"Sub"),
            ],
        }],
    };
