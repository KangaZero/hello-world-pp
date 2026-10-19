//! Hello-world snippet for `ProgrammingLanguageName::Zx`.
//!
//! Imported from leachim6/hello-world (`z/zx.js` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Zx => "zx",
    versions: [Zx => "zx"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zx,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zx,
            syntax: concat!(
                r#"#!/usr/bin/env zx
console.log(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    r"#!/usr/bin/env zx
",
                ),
                (TokenKind::Type, r"console"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"log"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
            ],
        }],
    };
