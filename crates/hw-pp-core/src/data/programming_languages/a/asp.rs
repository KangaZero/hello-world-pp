//! Hello-world snippet for `ProgrammingLanguageName::Asp`.
//!
//! Imported from leachim6/hello-world (`a/ASP.asp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Asp => "ASP",
    versions: [Asp => "ASP"],
    extensions: [Asp => "asp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Asp,
        extensions: &[FileExtension::Asp],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Asp,
            syntax: concat!(
                r#"<%@ Language= "VBScript" %>
<%
    Response.Write(""#,
                output_for_hello_world!(),
                r#"")
%>"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"<"),
                (TokenKind::Plain, r"%@ Language"),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r#""VBScript""#),
                (TokenKind::Plain, r" %"),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"<"),
                (
                    TokenKind::Plain,
                    r"%
    Response",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"Write"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
%",
                ),
                (TokenKind::Keyword, r">"),
            ],
        }],
    };
