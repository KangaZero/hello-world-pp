//! Hello-world snippet for `ProgrammingLanguageName::Rust`.
//!
//! Sources:
//! - <https://doc.rust-lang.org/book/ch01-02-hello-world.html>
//! - <https://doc.rust-lang.org/rust-by-example/hello.html>
//! - <https://doc.rust-lang.org/edition-guide/rust-2021/index.html>

use crate::data::programming_languages::ProgrammingLanguageFamily::{C, OCaml};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Rust => "Rust",
    versions: [Rust2021 => "Rust 2021"],
    extensions: [Rs => "rs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rust,
        extensions: &[FileExtension::Rs],
        categories: &[Compiled],
        families: &[C, OCaml],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rust2021,
            syntax: concat!(
                r#"fn main() {
    println!(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "fn"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, "()"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Macro, "println!"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ");"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    };
