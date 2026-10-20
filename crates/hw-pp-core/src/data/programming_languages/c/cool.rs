//! Hello-world snippet for `ProgrammingLanguageName::Cool`.
//!
//! Imported from leachim6/hello-world (`c/Cool.cl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cool.cl>

use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Educational},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cool => "Cool",
    versions: [Cool => "Cool"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cool,
        extensions: &[],
        categories: &[Compiled, Educational],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cool,
            syntax: concat!(
                r#"class Main inherits IO {
   main(): Object {
	out_string(""#,
                output_for_hello_world!(),
                r#".\n")
   };
};"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"class"),
                (TokenKind::Plain, r" Main inherits IO "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
   ",
                ),
                (TokenKind::Function, r"main"),
                (TokenKind::Punctuation, r"():"),
                (TokenKind::Plain, r" Object "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Function, r"out_string"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#".\n""#),
                ),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
   ",
                ),
                (TokenKind::Punctuation, r"};"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"};"),
            ],
        }],
    };
