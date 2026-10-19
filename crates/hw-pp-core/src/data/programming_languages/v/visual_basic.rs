//! Hello-world snippets for `ProgrammingLanguageName::VisualBasic`.
//!
//! Sources:
//! - <https://learn.microsoft.com/en-us/dotnet/visual-basic/getting-started/>
//! - <https://learn.microsoft.com/en-us/dotnet/visual-basic/programming-guide/program-structure/structure-of-a-visual-basic-program>

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
    versions: [VbNet => "VB.NET"],
    extensions: [Vb => "vb"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VisualBasic,
        extensions: &[FileExtension::Vb],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
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
        }],
    };
