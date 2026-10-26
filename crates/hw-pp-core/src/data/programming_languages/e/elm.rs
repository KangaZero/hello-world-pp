//! Hello-world snippet for `ProgrammingLanguageName::Elm`.
//!
//! Sources:
//! - <https://guide.elm-lang.org/>
//! - <https://package.elm-lang.org/packages/elm/html/latest/Html#text>
//! - <https://elm-lang.org/examples/hello>
//! - <https://www.wikidata.org/wiki/Q5366250> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Reactive};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Haskell, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Elm => "Elm",
    versions: [Elm019 => "Elm 0.19"],
    extensions: [Elm => "elm"],
}

const ELM019: &str = concat!(
    r#"module Main exposing (main)

import Html

main =
    Html.text ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elm,
        extensions: &[FileExtension::Elm],
        categories: &[Compiled],
        families: &[Haskell],
        paradigms: &[Functional, Reactive],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Elm019,
            syntax: ELM019,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ELM019,
                    [
                        (TokenKind::Keyword, "module"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Type, "Main"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Keyword, "exposing"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Punctuation, "("),
                        (TokenKind::Function, "main"),
                        (TokenKind::Punctuation, ")"),
                        (TokenKind::Plain, "\n\n"),
                        (TokenKind::Keyword, "import"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Type, "Html"),
                        (TokenKind::Plain, "\n\n"),
                        (TokenKind::Function, "main"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Punctuation, "="),
                        (TokenKind::Plain, "\n    "),
                        (TokenKind::Type, "Html"),
                        (TokenKind::Punctuation, "."),
                        (TokenKind::Function, "text"),
                        (TokenKind::Plain, " "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                    ],
                )
            },
        }],
    };
