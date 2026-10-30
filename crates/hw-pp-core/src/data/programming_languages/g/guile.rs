//! Hello-world snippet for `ProgrammingLanguageName::Guile`.
//!
//! Imported from the Hello World Collection (`files/Guile.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Guile.htm>
//! - <http://helloworldcollection.de/#Guile>
//! - <https://en.wikipedia.org/wiki/Scheme_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, Metaprogramming,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Guile => "Guile",
    versions: [Guile => "Guile"],
    extensions: [],
}

const GUILE: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r#" in Guile

(define do-hello (lambda () (display ""#,
    output_for_hello_world!(),
    r#".") (newline)))"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Guile,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, Functional, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Guile,
            syntax: GUILE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GUILE,
                    [
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
                )
            },
        }],
    };
