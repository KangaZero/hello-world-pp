//! Hello-world snippet for `ProgrammingLanguageName::Guile`.
//!
//! Imported from the Hello World Collection (`files/Guile.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Guile.htm>
//! - <http://helloworldcollection.de/#Guile>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Guile => "Guile",
    versions: [Guile => "Guile"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Guile,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Guile,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r#" in Guile

(define do-hello (lambda () (display ""#,
                output_for_hello_world!(),
                r#".") (newline)))"#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    concat!(r"; ", output_for_hello_world!(), r" in Guile"),
                ),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Keyword, r"define"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"do-hello"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Keyword, r"lambda"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Function, r"display"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#".""#),
                ),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Function, r"newline"),
                (TokenKind::Punctuation, r")))"),
            ],
        }],
    };
