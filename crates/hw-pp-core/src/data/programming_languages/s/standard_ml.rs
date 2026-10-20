//! Hello-world snippet for `ProgrammingLanguageName::StandardMl`.
//!
//! Imported from leachim6/hello-world (`s/Standard ML.sml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Standard%20ML.sml>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Ml, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: StandardMl => "Standard ML",
    versions: [StandardMl => "Standard ML"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::StandardMl,
        extensions: &[],
        categories: &[Compiled],
        families: &[Ml],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::StandardMl,
            syntax: concat!(
                r#"fun hello() = print(""#,
                output_for_hello_world!(),
                r#"\n");

hello()"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"fun"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"hello"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" = "),
                (TokenKind::Keyword, r"print"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Function, r"hello"),
                (TokenKind::Punctuation, r"()"),
            ],
        }],
    };
