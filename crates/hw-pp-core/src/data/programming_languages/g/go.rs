//! Hello-world snippet for `ProgrammingLanguageName::Go`.
//!
//! Sources:
//! - <https://go.dev/tour/welcome/1>
//! - <https://go.dev/doc/tutorial/getting-started>
//! - <https://go.dev/ref/spec>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::C, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Go => "Go",
    versions: [Go1 => "Go 1"],
    extensions: [Go => "go"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Go,
        extensions: &[FileExtension::Go],
        categories: &[Compiled],
        families: &[C],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Go1,
            syntax: concat!(
                r#"package main

import "fmt"

func main() {
    fmt.Println(""#,
                output_for_hello_world!(),
                r#"")
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "package"),
                (TokenKind::Plain, " main\n\n"),
                (TokenKind::Keyword, "import"),
                (TokenKind::Plain, " "),
                (TokenKind::String, "\"fmt\""),
                (TokenKind::Plain, "\n\n"),
                (TokenKind::Type, "func"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, "()"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    fmt"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Function, "Println"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ")"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    };
