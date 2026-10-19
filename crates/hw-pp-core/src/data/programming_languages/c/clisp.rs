//! Hello-world snippet for `ProgrammingLanguageName::Clisp`.
//!
//! Imported from leachim6/hello-world (`c/CLISP.lisp` at `a152253`),
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
    language: Clisp => "CLISP",
    versions: [Clisp => "CLISP"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clisp,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clisp,
            syntax: concat!(r#"(write-line ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Punctuation, r"("),
                (TokenKind::Function, r"write"),
                (TokenKind::Keyword, r"-"),
                (TokenKind::Plain, r"line "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
            ],
        }],
    };
