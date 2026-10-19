//! Hello-world snippets for `ProgrammingLanguageName::Scala`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Scala => "Scala",
    versions: [Scala3 => "Scala 3"],
    extensions: [Scala => "scala", Sc => "sc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Scala,
        extensions: &[FileExtension::Scala, FileExtension::Sc],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Scala3,
            syntax: concat!(
                r#"@main def hello(): Unit =
    println(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, "@"),
                (TokenKind::Plain, "main "),
                (TokenKind::Keyword, "def"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "hello"),
                (TokenKind::Plain, "()"),
                (TokenKind::Punctuation, ":"),
                (TokenKind::Plain, " "),
                (TokenKind::Type, "Unit"),
                (TokenKind::Plain, " "),
                (TokenKind::Keyword, "="),
                (TokenKind::Plain, "\n    println("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Plain, ")"),
            ],
        }],
    };
