//! Hello-world snippet for `ProgrammingLanguageName::Racket`.
//!
//! Sources:
//! - <https://docs.racket-lang.org/guide/intro.html>
//! - <https://docs.racket-lang.org/quick/>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Scripting},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Lisp,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Racket => "Racket",
    versions: [Racket8 => "Racket 8"],
    extensions: [Rkt => "rkt"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Racket,
        extensions: &[FileExtension::Rkt],
        categories: &[Compiled, Scripting],
        families: &[Lisp],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Racket8,
            syntax: concat!(
                r#"#lang racket
(displayln ""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "#lang"),
                (TokenKind::Plain, " racket\n"),
                (TokenKind::Punctuation, "("),
                (TokenKind::Function, "displayln"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ")"),
            ],
        }],
    };
