//! Hello-world snippet for `ProgrammingLanguageName::Kotlin`.
//!
//! Sources:
//! - <https://kotlinlang.org/docs/kotlin-tour-hello-world.html>
//! - <https://kotlinlang.org/docs/whatsnew19.html>

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
    language: Kotlin => "Kotlin",
    versions: [Kotlin19 => "Kotlin 1.9"],
    extensions: [Kt => "kt", Kts => "kts"],
}

const KOTLIN19: &str = concat!(
    r#"fun main() {
    println(""#,
    output_for_hello_world!(),
    r#"")
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kotlin,
        extensions: &[FileExtension::Kt, FileExtension::Kts],
        categories: &[Compiled],
        families: &[Java],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kotlin19,
            syntax: KOTLIN19,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KOTLIN19,
                    [
                        (TokenKind::Keyword, "fun"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Function, "main"),
                        (TokenKind::Punctuation, "()"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Punctuation, "{"),
                        (TokenKind::Plain, "\n    "),
                        (TokenKind::Function, "println"),
                        (TokenKind::Punctuation, "("),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                        (TokenKind::Punctuation, ")"),
                        (TokenKind::Plain, "\n"),
                        (TokenKind::Punctuation, "}"),
                    ],
                )
            },
        }],
    };
