//! Hello-world snippet for `ProgrammingLanguageName::Ironscheme`.
//!
//! Imported from leachim6/hello-world (`i/IronScheme.ss` at `a152253`),
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
    language: Ironscheme => "IronScheme",
    versions: [Ironscheme => "IronScheme"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ironscheme,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ironscheme,
            syntax: concat!(
                r#"(import (rnrs))

(display ""#,
                output_for_hello_world!(),
                r#"")
(newline)"#
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, r"("),
                (TokenKind::Function, r"import"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"rnrs"),
                (TokenKind::Punctuation, r"))"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"display "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"newline"),
                (TokenKind::Punctuation, r")"),
            ],
        }],
    };
