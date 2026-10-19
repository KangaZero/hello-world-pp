//! Hello-world snippet for `ProgrammingLanguageName::VbaWord`.
//!
//! Imported from the Hello World Collection (`files/VBA&nbsp;(Word).htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/VBA%26nbsp%3B%28Word%29.htm>
//! - <http://helloworldcollection.de/#VBA%C2%A0(Word)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: VbaWord => "VBA (Word)",
    versions: [VbaWord => "VBA (Word)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VbaWord,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VbaWord,
            syntax: concat!(
                r"' ",
                output_for_hello_world!(),
                r#" in Visual Basic for Applications, Word version

Private Sub Document_Open()
    MsgBox ""#,
                output_for_hello_world!(),
                r#""
End Sub"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"' ",
                    output_for_hello_world!(),
                    r#" in Visual Basic for Applications, Word version

Private Sub Document_Open()
    MsgBox ""#,
                    output_for_hello_world!(),
                    r#""
End Sub"#
                ),
            )],
        }],
    };
