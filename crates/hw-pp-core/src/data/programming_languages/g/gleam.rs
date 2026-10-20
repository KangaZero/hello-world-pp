//! Hello-world snippet for `ProgrammingLanguageName::Gleam`.
//!
//! Imported from leachim6/hello-world (`g/Gleam.gleam` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Gleam.gleam>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Gleam => "Gleam",
    versions: [Gleam => "Gleam"],
    extensions: [Gleam => "gleam"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gleam,
        extensions: &[FileExtension::Gleam],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gleam,
            syntax: concat!(
                r#"import gleam/io

pub fn main() {
  io.println(""#,
                output_for_hello_world!(),
                r#"")
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" gleam/io

",
                ),
                (TokenKind::Keyword, r"pub"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"fn"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"main"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
  io",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"println"),
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
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };
