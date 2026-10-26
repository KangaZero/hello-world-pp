//! Hello-world snippet for `ProgrammingLanguageName::Express`.
//!
//! Imported from leachim6/hello-world (`e/Express.js` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Express.js>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Express => "Express",
    versions: [Express => "Express"],
    extensions: [],
}

const EXPRESS: &str = concat!(
    r#"const express = require('express')
const app = express()

app.get('/', (_, res) => res.send(""#,
    output_for_hello_world!(),
    r#""))

app.listen(8080)"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Express,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Express,
            syntax: EXPRESS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EXPRESS,
                    [
                        (TokenKind::Type, r"const"),
                        (TokenKind::Plain, r" express "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"require"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r"'express'"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Type, r"const"),
                        (TokenKind::Plain, r" app "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"express"),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"

app",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"get"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r"'/'"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"_"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" res"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"=>"),
                        (TokenKind::Plain, r" res"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"send"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r"))"),
                        (
                            TokenKind::Plain,
                            r"

app",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"listen"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Number, r"8080"),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
