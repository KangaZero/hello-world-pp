//! Hello-world snippet for `ProgrammingLanguageName::ErlangEscript`.
//!
//! Imported from leachim6/hello-world (`e/Erlang EScript.erl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Erlang%20EScript.erl>
//! - <https://en.wikipedia.org/wiki/Erlang_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Concurrent, Functional};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: ErlangEscript => "Erlang EScript",
    versions: [ErlangEscript => "Erlang EScript"],
    extensions: [],
}

const ERLANG_ESCRIPT: &str = concat!(
    r#"#!/usr/bin/env escript

main(_) ->
    io:format(""#,
    output_for_hello_world!(),
    r#"~n")."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ErlangEscript,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Functional, Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ErlangEscript,
            syntax: ERLANG_ESCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ERLANG_ESCRIPT,
                    [
                        (TokenKind::Plain, r"#"),
                        (TokenKind::Keyword, r"!/"),
                        (TokenKind::Plain, r"usr"),
                        (TokenKind::Keyword, r"/"),
                        (TokenKind::Plain, r"bin"),
                        (TokenKind::Keyword, r"/"),
                        (
                            TokenKind::Plain,
                            r"env escript

",
                        ),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"_"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"->"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Type, r"io"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Function, r"format"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"~n""#),
                        ),
                        (TokenKind::Punctuation, r")."),
                    ],
                )
            },
        }],
    };
