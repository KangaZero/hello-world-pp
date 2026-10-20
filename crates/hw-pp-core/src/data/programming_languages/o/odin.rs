//! Hello-world snippet for `ProgrammingLanguageName::Odin`.
//!
//! Imported from leachim6/hello-world (`o/Odin.odin` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Odin.odin>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Odin => "Odin",
    versions: [Odin => "Odin"],
    extensions: [Odin => "odin"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Odin,
        extensions: &[FileExtension::Odin],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Odin,
            syntax: concat!(
                r#"package main

import "core:fmt"

main :: proc() {
  fmt.println(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"package"),
                (
                    TokenKind::Plain,
                    r" main

",
                ),
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r#""core:fmt""#),
                (
                    TokenKind::Plain,
                    r"

main ",
                ),
                (TokenKind::Punctuation, r"::"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"proc"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
  fmt",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"println"),
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
            ],
        }],
    };
