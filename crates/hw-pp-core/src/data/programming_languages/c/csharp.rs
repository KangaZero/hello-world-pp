//! Hello-world snippets for `ProgrammingLanguageName::CSharp`.
//!
//! Sources:
//! - <https://learn.microsoft.com/en-us/dotnet/csharp/tour-of-csharp/tutorials/hello-world>
//! - <https://learn.microsoft.com/en-us/dotnet/csharp/fundamentals/program-structure/top-level-statements>
//! - <https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: CSharp => "C#",
    versions: [CSharp10 => "C# 10"],
    extensions: [Cs => "cs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CSharp,
        extensions: &[FileExtension::Cs],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CSharp10,
            syntax: concat!(
                r#"Console.WriteLine(""#,
                output_for_hello_world!(),
                r#"");"#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, "Console"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Function, "WriteLine"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ");"),
            ],
        }],
    };
