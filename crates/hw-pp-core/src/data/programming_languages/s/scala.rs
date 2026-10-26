//! Hello-world snippet for `ProgrammingLanguageName::Scala`.
//!
//! Sources:
//! - <https://docs.scala-lang.org/scala3/book/taste-hello-world.html>
//! - <https://docs.scala-lang.org/scala3/book/methods-main-methods.html>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Java, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Scala => "Scala",
    versions: [Scala3 => "Scala 3"],
    extensions: [Scala => "scala", Sc => "sc"],
}

const SCALA3: &str = concat!(
    r#"@main def hello(): Unit =
    println(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Scala,
        extensions: &[FileExtension::Scala, FileExtension::Sc],
        categories: &[Compiled],
        families: &[Java],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Scala3,
            syntax: SCALA3,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SCALA3,
                    [
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
                )
            },
        }],
    };
