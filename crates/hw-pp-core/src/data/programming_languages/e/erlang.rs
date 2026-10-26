//! Hello-world snippet for `ProgrammingLanguageName::Erlang`.
//!
//! Sources:
//! - <https://www.erlang.org/doc/system/seq_prog.html>
//! - <https://www.erlang.org/doc/apps/stdlib/io.html>
//! - <https://www.wikidata.org/wiki/Q334879> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Concurrent, Declarative, Functional,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Erlang => "Erlang",
    versions: [ErlangOtp => "Erlang/OTP"],
    extensions: [Erl => "erl", Hrl => "hrl"],
}

const ERLANG_OTP: &str = concat!(
    r#"-module(hello).
-export([main/0]).

main() ->
    io:format(""#,
    output_for_hello_world!(),
    r#"~n")."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Erlang,
        extensions: &[FileExtension::Erl, FileExtension::Hrl],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Functional, Declarative, Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ErlangOtp,
            syntax: ERLANG_OTP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ERLANG_OTP,
                    [
                        (TokenKind::Punctuation, "-"),
                        (TokenKind::Keyword, "module"),
                        (TokenKind::Punctuation, "("),
                        (TokenKind::Type, "hello"),
                        (TokenKind::Punctuation, ")."),
                        (TokenKind::Plain, "\n"),
                        (TokenKind::Punctuation, "-"),
                        (TokenKind::Keyword, "export"),
                        (TokenKind::Punctuation, "(["),
                        (TokenKind::Function, "main"),
                        (TokenKind::Punctuation, "/"),
                        (TokenKind::Number, "0"),
                        (TokenKind::Punctuation, "])."),
                        (TokenKind::Plain, "\n\n"),
                        (TokenKind::Function, "main"),
                        (TokenKind::Punctuation, "()"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Keyword, "->"),
                        (TokenKind::Plain, "\n    "),
                        (TokenKind::Type, "io"),
                        (TokenKind::Punctuation, ":"),
                        (TokenKind::Function, "format"),
                        (TokenKind::Punctuation, "("),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "~n\""),
                        ),
                        (TokenKind::Punctuation, ")."),
                    ],
                )
            },
        }],
    };
