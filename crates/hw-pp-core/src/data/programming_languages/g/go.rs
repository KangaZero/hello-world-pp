//! Hello-world snippet for `ProgrammingLanguageName::Go`.
//!
//! Sources:
//! - <https://go.dev/tour/welcome/1>
//! - <https://go.dev/doc/tutorial/getting-started>
//! - <https://go.dev/ref/spec>
//! - <https://www.wikidata.org/wiki/Q37227> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Concurrent, Functional, Imperative, ObjectOriented, Procedural, Structured,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::C, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Go => "Go",
    versions: [Go1 => "Go 1"],
    extensions: [Go => "go"],
}

const GO1: &str = concat!(
    r#"package main

import "fmt"

func main() {
    fmt.Println(""#,
    output_for_hello_world!(),
    r#"")
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Go,
        extensions: &[FileExtension::Go],
        categories: &[Compiled],
        families: &[C],
        paradigms: &[
            Imperative,
            Procedural,
            Structured,
            ObjectOriented,
            Functional,
            Concurrent,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Go1,
            syntax: GO1,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GO1,
                    [
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
                )
            },
        }],
    };
