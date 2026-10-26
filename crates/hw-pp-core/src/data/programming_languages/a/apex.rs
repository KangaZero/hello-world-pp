//! Hello-world snippet for `ProgrammingLanguageName::Apex`.
//!
//! Imported from leachim6/hello-world (`a/Apex.cls` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Apex.cls>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Apex => "Apex",
    versions: [Apex => "Apex"],
    extensions: [Cls => "cls"],
}

const APEX: &str = concat!(
    r"global with sharing class HelloWorld {
  global static void main() {
    System.debug('",
    output_for_hello_world!(),
    r"');
  }
}"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Apex,
        extensions: &[FileExtension::Cls],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Apex,
            syntax: APEX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    APEX,
                    [
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
                )
            },
        }],
    };
