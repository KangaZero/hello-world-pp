//! Hello-world snippets for `ProgrammingLanguageName::CSharp`.
//!
//! Sources:
//! - <https://learn.microsoft.com/en-us/dotnet/csharp/tour-of-csharp/tutorials/hello-world>
//! - <https://learn.microsoft.com/en-us/dotnet/csharp/fundamentals/program-structure/top-level-statements>
//! - <https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history>
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CSharp.cs>
//!
//! The `C# 9` version is imported from leachim6/hello-world,
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

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
    versions: [CSharp10 => "C# 10", CSharp9 => "C# 9"],
    extensions: [Cs => "cs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CSharp,
        extensions: &[FileExtension::Cs],
        categories: &[Compiled],
        versions: &[
            ProgrammingLanguageVersion {
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
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CSharp9,
                syntax: concat!(
                    r#"System.Console.WriteLine(""#,
                    output_for_hello_world!(),
                    r#"");"#
                ),
                tokenized_syntax: &[
                    (TokenKind::Plain, r"System"),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Plain, r"Console"),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"WriteLine"),
                    (TokenKind::Punctuation, r"("),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r");"),
                ],
            },
        ],
    };
