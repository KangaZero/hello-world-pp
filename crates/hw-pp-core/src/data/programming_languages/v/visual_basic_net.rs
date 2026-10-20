//! Hello-world snippet for `ProgrammingLanguageName::VisualBasicNet`.
//!
//! Imported from the Hello World Collection (`files/Visual&nbsp;Basic&nbsp;.NET.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Visual%26nbsp%3BBasic%26nbsp%3B.NET.txt>
//! - <http://helloworldcollection.de/#Visual%C2%A0Basic%C2%A0.NET>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: VisualBasicNet => "Visual Basic .NET",
    versions: [VisualBasicNet => "Visual Basic .NET"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VisualBasicNet,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VisualBasicNet,
            syntax: concat!(
                r"'",
                output_for_hello_world!(),
                r#" in Visual Basic .NET (VB.NET)

Imports System.Console

Class HelloWorld

    Public Shared Sub Main()
        WriteLine(""#,
                output_for_hello_world!(),
                r#"")
    End Sub

End Class"#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    concat!(
                        r"'",
                        output_for_hello_world!(),
                        r" in Visual Basic .NET (VB.NET)"
                    ),
                ),
                (
                    TokenKind::Plain,
                    r"

Imports System",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Keyword, r"Console"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"Class"),
                (
                    TokenKind::Plain,
                    r" HelloWorld

    ",
                ),
                (TokenKind::Keyword, r"Public"),
                (TokenKind::Plain, r" Shared "),
                (TokenKind::Keyword, r"Sub"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"Main"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"WriteLine"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"End"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"Sub"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"End"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"Class"),
            ],
        }],
    };
