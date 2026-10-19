//! Hello-world snippet for `ProgrammingLanguageName::Apex`.
//!
//! Imported from leachim6/hello-world (`a/Apex.cls` at `a152253`),
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
    language: Apex => "Apex",
    versions: [Apex => "Apex"],
    extensions: [Cls => "cls"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Apex,
        extensions: &[FileExtension::Cls],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Apex,
            syntax: concat!(
                r"global with sharing class HelloWorld {
  global static void main() {
    System.debug('",
                output_for_hello_world!(),
                r"');
  }
}"
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, r"global with sharing "),
                (TokenKind::Type, r"class"),
                (TokenKind::Plain, r" HelloWorld "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
  global ",
                ),
                (TokenKind::Keyword, r"static"),
                (TokenKind::Plain, r" "),
                (TokenKind::Type, r"void"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"main"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    System",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"debug"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r"'", output_for_hello_world!(), r"'"),
                ),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };
