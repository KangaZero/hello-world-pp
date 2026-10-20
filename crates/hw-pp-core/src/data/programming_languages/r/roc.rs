//! Hello-world snippet for `ProgrammingLanguageName::Roc`.
//!
//! Imported from leachim6/hello-world (PR #1598 `r/Roc.roc`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1598>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Roc => "Roc",
    versions: [Roc => "Roc"],
    extensions: [Roc => "roc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Roc,
        extensions: &[FileExtension::Roc],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Roc,
            syntax: concat!(
                r#"app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.12.0/Lb8EgiejTUzbggO2HVVuPJFkwvvsfW6LojkLR20kTVE.tar.br" }

import pf.Stdout
import pf.Task

main =
    Stdout.line! ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, r"app "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"main"),
                (TokenKind::Punctuation, r"]"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (TokenKind::Plain, r" pf"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" platform "),
                (
                    TokenKind::String,
                    r#""https://github.com/roc-lang/basic-cli/releases/download/0.12.0/Lb8EgiejTUzbggO2HVVuPJFkwvvsfW6LojkLR20kTVE.tar.br""#,
                ),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" pf"),
                (TokenKind::Punctuation, r"."),
                (
                    TokenKind::Plain,
                    r"Stdout
",
                ),
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" pf"),
                (TokenKind::Punctuation, r"."),
                (
                    TokenKind::Plain,
                    r"Task

main =
    Stdout",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"line! "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
            ],
        }],
    };
