//! Hello-world snippet for `ProgrammingLanguageName::Islisp`.
//!
//! Imported from leachim6/hello-world (`i/ISLISP.lisp` at `a152253`),
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
    language: Islisp => "ISLISP",
    versions: [Islisp => "ISLISP"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Islisp,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Islisp,
            syntax: concat!(
                r#"(format (standard-output) ""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, r"("),
                (TokenKind::Function, r"format"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"standard"),
                (TokenKind::Keyword, r"-"),
                (TokenKind::Plain, r"output"),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
            ],
        }],
    };
