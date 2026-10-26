//! Hello-world snippet for `ProgrammingLanguageName::Flet`.
//!
//! Imported from leachim6/hello-world (PR #1519 `f/flet.py`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1519>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Flet => "Flet",
    versions: [Flet => "Flet"],
    extensions: [],
}

const FLET: &str = concat!(
    r#"import flet as ft

def main(page: ft.Page):
    t = ft.Text(value=""#,
    output_for_hello_world!(),
    r#"")
    page.controls.append(t)
    page.update()

ft.app(target=main)"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Flet,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Flet,
            syntax: FLET,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FLET,
                    [
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" flet "),
                        (TokenKind::Keyword, r"as"),
                        (
                            TokenKind::Plain,
                            r" ft

",
                        ),
                        (TokenKind::Keyword, r"def"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"page"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" ft"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"Page"),
                        (TokenKind::Punctuation, r"):"),
                        (
                            TokenKind::Plain,
                            r"
    t ",
                        ),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" ft"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"Text"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"value"),
                        (TokenKind::Keyword, r"="),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
    page",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"controls"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"append"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"t"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
    page",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"update"),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"

ft",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"app"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"target"),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r"main"),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
