//! Hello-world snippet for `ProgrammingLanguageName::Purescript`.
//!
//! Imported from leachim6/hello-world (`p/PureScript.purs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PureScript.purs>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Haskell, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Purescript => "PureScript",
    versions: [Purescript => "PureScript"],
    extensions: [Purs => "purs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Purescript,
        extensions: &[FileExtension::Purs],
        categories: &[Compiled],
        families: &[Haskell],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Purescript,
            syntax: concat!(
                r#"module Main where

import Debug.Trace

main = trace ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"module"),
                (TokenKind::Plain, r" Main "),
                (TokenKind::Keyword, r"where"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" Debug"),
                (TokenKind::Punctuation, r"."),
                (
                    TokenKind::Plain,
                    r"Trace

main = trace ",
                ),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
            ],
        }],
    };
