//! Hello-world snippet for `ProgrammingLanguageName::Curry`.
//!
//! Imported from leachim6/hello-world (`c/Curry.curry` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Curry.curry>

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
    language: Curry => "Curry",
    versions: [Curry => "Curry"],
    extensions: [Curry => "curry"],
}

const CURRY: &str = concat!(
    r#"-- ""#,
    output_for_hello_world!(),
    r#"" demo for the Tcl/Tk library

import Tk

main = runWidget "Hello"
          (TkCol [] [TkLabel [TkText ""#,
    output_for_hello_world!(),
    r#""],
                     TkButton tkExit [TkText "Stop"]])"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Curry,
        extensions: &[FileExtension::Curry],
        categories: &[Compiled],
        families: &[Haskell],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Curry,
            syntax: CURRY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CURRY,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r#"-- ""#,
                                output_for_hello_world!(),
                                r#"" demo for the Tcl/Tk library"#
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"import"),
                        (
                            TokenKind::Plain,
                            r" Tk

main = runWidget ",
                        ),
                        (TokenKind::String, r#""Hello""#),
                        (
                            TokenKind::Plain,
                            r"
          ",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"TkCol "),
                        (TokenKind::Punctuation, r"[]"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Plain, r"TkLabel "),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Plain, r"TkText "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r"],"),
                        (
                            TokenKind::Plain,
                            r"
                     TkButton tkExit ",
                        ),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Plain, r"TkText "),
                        (TokenKind::String, r#""Stop""#),
                        (TokenKind::Punctuation, r"]])"),
                    ],
                )
            },
        }],
    };
