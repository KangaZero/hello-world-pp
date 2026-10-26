//! Hello-world snippet for `ProgrammingLanguageName::Mercury`.
//!
//! Imported from leachim6/hello-world (`m/Mercury.m` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mercury.m>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mercury => "Mercury",
    versions: [Mercury => "Mercury"],
    extensions: [],
}

const MERCURY: &str = concat!(
    r#":- module hello.
:- interface.
:- import_module io.
:- pred main(io::di, io::uo) is det.

:- implementation.
main(!IO) :-
	io.write_string(""#,
    output_for_hello_world!(),
    r#"\n", !IO)."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mercury,
        extensions: &[],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mercury,
            syntax: MERCURY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MERCURY,
                    [
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"- "),
                        (TokenKind::Keyword, r"module"),
                        (TokenKind::Plain, r" hello"),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"- "),
                        (TokenKind::Keyword, r"interface"),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"- import_module "),
                        (TokenKind::Keyword, r"io"),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"- "),
                        (TokenKind::Keyword, r"pred"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"main"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"io"),
                        (TokenKind::Punctuation, r"::"),
                        (TokenKind::Plain, r"di"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"io"),
                        (TokenKind::Punctuation, r"::"),
                        (TokenKind::Plain, r"uo"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"is"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"det"),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"- implementation"),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"main"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"!IO"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"-
	",
                        ),
                        (TokenKind::Keyword, r"io"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"write_string"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" !IO"),
                        (TokenKind::Punctuation, r")."),
                    ],
                )
            },
        }],
    };
