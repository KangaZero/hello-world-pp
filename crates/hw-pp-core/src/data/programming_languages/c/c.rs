//! Hello-world snippets for `ProgrammingLanguageName::C`.
//!
//! Sources:
//! - <https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1256.pdf>
//! - <https://en.wikipedia.org/wiki/C99>
//! - <https://en.cppreference.com/c/io/printf>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: C => "C",
    versions: [C99 => "C99"],
    extensions: [C => "c", H => "h"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::C,
        extensions: &[FileExtension::C, FileExtension::H],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::C99,
            syntax: concat!(
                r#"#include <stdio.h>

int main(void) {
    printf(""#,
                output_for_hello_world!(),
                r#"\n");
    return 0;
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "#include"),
                (TokenKind::Plain, " "),
                (TokenKind::String, "<stdio.h>"),
                (TokenKind::Plain, "\n\n"),
                (TokenKind::Type, "int"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, "("),
                (TokenKind::Type, "void"),
                (TokenKind::Punctuation, ")"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Function, "printf"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Punctuation, ");"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Keyword, "return"),
                (TokenKind::Plain, " "),
                (TokenKind::Number, "0"),
                (TokenKind::Punctuation, ";"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    };
