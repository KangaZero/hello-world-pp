//! Hello-world snippets for `ProgrammingLanguageName::Elm`.
//!
//! Sources:
//! - <https://guide.elm-lang.org/>
//! - <https://package.elm-lang.org/packages/elm/html/latest/Html#text>
//! - <https://elm-lang.org/examples/hello>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Elm => "Elm",
    versions: [Elm019 => "Elm 0.19"],
    extensions: [Elm => "elm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elm,
        extensions: &[FileExtension::Elm],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Elm019,
            syntax: concat!(
                r#"module Main exposing (main)

import Html

main =
    Html.text ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
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
        }],
    };
