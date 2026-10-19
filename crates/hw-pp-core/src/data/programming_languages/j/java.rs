//! Hello-world snippets for `ProgrammingLanguageName::Java`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Java => "Java",
    versions: [Java17 => "Java 17", Java25CompactSourceFile => "Java 25 (compact source file)"],
    extensions: [Java => "java"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Java,
        extensions: &[FileExtension::Java],
        categories: &[Compiled],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Java17,
                syntax: concat!(
                    r#"class Main {
    public static void main(String[] args) {
        System.out.println(""#,
                    output_for_hello_world!(),
                    r#"");
    }
}"#
                ),
                tokenized_syntax: &[
                    (TokenKind::Type, "class"),
                    (TokenKind::Plain, " Main "),
                    (TokenKind::Punctuation, "{"),
                    (TokenKind::Plain, "\n    "),
                    (TokenKind::Keyword, "public"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Keyword, "static"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Type, "void"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Function, "main"),
                    (TokenKind::Punctuation, "("),
                    (TokenKind::Plain, "String"),
                    (TokenKind::Keyword, "[]"),
                    (TokenKind::Plain, " args"),
                    (TokenKind::Punctuation, ")"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Punctuation, "{"),
                    (TokenKind::Plain, "\n        System"),
                    (TokenKind::Punctuation, "."),
                    (TokenKind::Plain, "out"),
                    (TokenKind::Punctuation, "."),
                    (TokenKind::Function, "println"),
                    (TokenKind::Punctuation, "("),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), "\""),
                    ),
                    (TokenKind::Punctuation, ");"),
                    (TokenKind::Plain, "\n    "),
                    (TokenKind::Punctuation, "}"),
                    (TokenKind::Plain, "\n"),
                    (TokenKind::Punctuation, "}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Java25CompactSourceFile,
                syntax: concat!(
                    r#"void main() {
    IO.println(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
                tokenized_syntax: &[
                    (TokenKind::Type, "void"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Function, "main"),
                    (TokenKind::Punctuation, "()"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Punctuation, "{"),
                    (TokenKind::Plain, "\n    IO"),
                    (TokenKind::Punctuation, "."),
                    (TokenKind::Function, "println"),
                    (TokenKind::Punctuation, "("),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), "\""),
                    ),
                    (TokenKind::Punctuation, ");"),
                    (TokenKind::Plain, "\n"),
                    (TokenKind::Punctuation, "}"),
                ],
            },
        ],
    };
