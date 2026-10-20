//! Hello-world snippet for `ProgrammingLanguageName::Dockerfile`.
//!
//! Imported from leachim6/hello-world (PR #1681 `d/dockerfile`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1681>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Configuration, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dockerfile => "Dockerfile",
    versions: [Dockerfile => "Dockerfile"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dockerfile,
        extensions: &[],
        categories: &[Configuration],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dockerfile,
            syntax: concat!(
                r#"FROM alpine:latest
CMD echo ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"FROM"),
                (TokenKind::Plain, r" alpine"),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"latest
",
                ),
                (TokenKind::Keyword, r"CMD"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"echo"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
            ],
        }],
    };
