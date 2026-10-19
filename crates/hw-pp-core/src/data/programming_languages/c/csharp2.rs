//! Hello-world snippet for `ProgrammingLanguageName::Csharp`.
//!
//! Imported from leachim6/hello-world (`c/CSharp.cs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Csharp => "CSharp",
    versions: [Csharp => "CSharp"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Csharp,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Csharp,
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
        }],
    };
