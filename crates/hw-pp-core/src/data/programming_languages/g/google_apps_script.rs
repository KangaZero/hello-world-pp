//! Hello-world snippet for `ProgrammingLanguageName::GoogleAppsScript`.
//!
//! Imported from leachim6/hello-world (`g/Google Apps Script.gs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Google%20Apps%20Script.gs>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: GoogleAppsScript => "Google Apps Script",
    versions: [GoogleAppsScript => "Google Apps Script"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GoogleAppsScript,
        extensions: &[],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GoogleAppsScript,
            syntax: concat!(
                r#"function helloWorld() {
  Logger.log(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"function"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"helloWorld"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
  Logger",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"log"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };
