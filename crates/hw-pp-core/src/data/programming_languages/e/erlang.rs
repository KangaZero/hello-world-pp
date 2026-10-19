//! Hello-world snippets for `ProgrammingLanguageName::Erlang`.
//!
//! Sources:
//! - <https://www.erlang.org/doc/system/seq_prog.html>
//! - <https://www.erlang.org/doc/apps/stdlib/io.html>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Erlang => "Erlang",
    versions: [ErlangOtp => "Erlang/OTP"],
    extensions: [Erl => "erl", Hrl => "hrl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Erlang,
        extensions: &[FileExtension::Erl, FileExtension::Hrl],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ErlangOtp,
            syntax: concat!(
                r#"-module(hello).
-export([main/0]).

main() ->
    io:format(""#,
                output_for_hello_world!(),
                r#"~n")."#
            ),
            tokenized_syntax: &[
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
        }],
    };
