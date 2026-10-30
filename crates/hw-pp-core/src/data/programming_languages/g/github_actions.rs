//! Hello-world snippet for `ProgrammingLanguageName::GithubActions`.
//!
//! Imported from leachim6/hello-world (`g/GitHub Actions.yaml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/GitHub%20Actions.yaml>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Configuration,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: GithubActions => "GitHub Actions",
    versions: [GithubActions => "GitHub Actions"],
    extensions: [Yaml => "yaml"],
}

const GITHUB_ACTIONS: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GithubActions,
        extensions: &[FileExtension::Yaml],
        categories: &[Configuration],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GithubActions,
            syntax: GITHUB_ACTIONS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GITHUB_ACTIONS,
                    [
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
                )
            },
        }],
    };
