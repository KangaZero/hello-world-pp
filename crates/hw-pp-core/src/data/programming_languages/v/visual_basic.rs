//! Hello-world snippets for `ProgrammingLanguageName::VisualBasic`.
//!
//! Sources:
//! - <https://learn.microsoft.com/en-us/dotnet/visual-basic/getting-started/>
//! - <https://learn.microsoft.com/en-us/dotnet/visual-basic/programming-guide/program-structure/structure-of-a-visual-basic-program>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Visual%26nbsp%3BBasic%26nbsp%3B6.txt>
//! - <http://helloworldcollection.de/#Visual%C2%A0Basic%C2%A06>
//!
//! The `Visual Basic 6` version is imported from the Hello World Collection, MIT licensed; see
//! `programming_languages/LICENSE-helloworldcollection`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: VisualBasic => "Visual Basic",
    versions: [VbNet => "VB.NET", VisualBasic6 => "Visual Basic 6"],
    extensions: [Vb => "vb"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VisualBasic,
        extensions: &[FileExtension::Vb],
        categories: &[Compiled],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::VbNet,
                syntax: concat!(
                    r#"Module Program
    Sub Main()
        Console.WriteLine(""#,
                    output_for_hello_world!(),
                    r#"")
    End Sub
End Module"#
                ),
                tokenized_syntax: &[
                    (TokenKind::Keyword, "Module"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Type, "Program"),
                    (TokenKind::Plain, "\n    "),
                    (TokenKind::Keyword, "Sub"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Function, "Main"),
                    (TokenKind::Punctuation, "()"),
                    (TokenKind::Plain, "\n        "),
                    (TokenKind::Type, "Console"),
                    (TokenKind::Punctuation, "."),
                    (TokenKind::Function, "WriteLine"),
                    (TokenKind::Punctuation, "("),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), "\""),
                    ),
                    (TokenKind::Punctuation, ")"),
                    (TokenKind::Plain, "\n    "),
                    (TokenKind::Keyword, "End"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Keyword, "Sub"),
                    (TokenKind::Plain, "\n"),
                    (TokenKind::Keyword, "End"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Keyword, "Module"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::VisualBasic6,
                syntax: concat!(
                    r"' ",
                    output_for_hello_world!(),
                    r#" in Visual Basic 6

Private Sub Form_Load()
Print ""#,
                    output_for_hello_world!(),
                    r#""
End Sub"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!("' ", output_for_hello_world!(), " in Visual Basic 6"),
                    ),
                    (TokenKind::Plain, "\n\n"),
                    (TokenKind::Keyword, "Private"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Keyword, "Sub"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Function, "Form_Load"),
                    (TokenKind::Punctuation, "()"),
                    (TokenKind::Plain, "\n"),
                    (TokenKind::Keyword, "Print"),
                    (TokenKind::Plain, " "),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), "\""),
                    ),
                    (TokenKind::Plain, "\n"),
                    (TokenKind::Keyword, "End"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Keyword, "Sub"),
                ],
            },
        ],
    };
