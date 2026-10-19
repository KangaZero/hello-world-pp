//! Hello-world snippet for `ProgrammingLanguageName::GithubActions`.
//!
//! Imported from leachim6/hello-world (`g/GitHub Actions.yaml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: GithubActions => "GitHub Actions",
    versions: [GithubActions => "GitHub Actions"],
    extensions: [Yaml => "yaml"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GithubActions,
        extensions: &[FileExtension::Yaml],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GithubActions,
            syntax: concat!(
                r"name: ",
                output_for_hello_world!(),
                r#"

on:
  workflow_dispatch

jobs:
  hello:
    runs-on: ubuntu-latest
    steps:
      - run: echo ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (TokenKind::String, r"name"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, output_for_hello_world!()),
                (
                    TokenKind::Plain,
                    r"

on",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::String, r"workflow_dispatch"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::String, r"jobs"),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::String, r"hello"),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::String, r"runs-on"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r"ubuntu-latest"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::String, r"steps"),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
      ",
                ),
                (TokenKind::Punctuation, r"-"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r"run"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"echo ""#, output_for_hello_world!(), r#"""#),
                ),
            ],
        }],
    };
