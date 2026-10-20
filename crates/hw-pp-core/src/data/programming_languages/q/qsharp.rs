//! Hello-world snippet for `ProgrammingLanguageName::Qsharp`.
//!
//! Imported from leachim6/hello-world (`q/QSharp.qs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/QSharp.qs>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Qsharp => "QSharp",
    versions: [Qsharp => "QSharp"],
    extensions: [Qs => "qs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qsharp,
        extensions: &[FileExtension::Qs],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qsharp,
            syntax: concat!(
                "\u{feff}namespace Quantum.HelloWorld {\n    open Microsoft.Quantum.Canon;\n    open Microsoft.Quantum.Intrinsic;\n\n\n    operation HelloWorld() : Unit {\n        Message(\"",
                output_for_hello_world!(),
                r#"");
    }
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, "\u{feff}"),
                (TokenKind::Keyword, r"namespace"),
                (TokenKind::Plain, r" Quantum"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"HelloWorld "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"open"),
                (TokenKind::Plain, r" Microsoft"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"Quantum"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"Canon"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"open"),
                (TokenKind::Plain, r" Microsoft"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"Quantum"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"Intrinsic"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"


    operation ",
                ),
                (TokenKind::Function, r"HelloWorld"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (TokenKind::Type, r"Unit"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Function, r"Message"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };
