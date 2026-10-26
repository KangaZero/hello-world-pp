//! Hello-world snippet for `ProgrammingLanguageName::Newlisp`.
//!
//! Imported from leachim6/hello-world (`n/NewLISP.lsp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/NewLISP.lsp>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Lisp, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Newlisp => "NewLISP",
    versions: [Newlisp => "NewLISP"],
    extensions: [],
}

const NEWLISP: &str = concat!(
    r#"#!/usr/bin/newlisp
(print ""#,
    output_for_hello_world!(),
    r#"\n")
(exit)"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Newlisp,
        extensions: &[],
        categories: &[Scripting],
        families: &[Lisp],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Newlisp,
            syntax: NEWLISP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NEWLISP,
                    [
                        (
                            TokenKind::Plain,
                            r"#!/usr/bin/newlisp
",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"exit"),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
