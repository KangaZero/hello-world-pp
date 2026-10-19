//! Hello-world snippets for `ProgrammingLanguageName::Dart`.
//!
//! Sources:
//! - <https://dart.dev/language>
//! - <https://dart.dev/overview>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Scripting},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dart => "Dart",
    versions: [Dart3 => "Dart 3"],
    extensions: [Dart => "dart"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dart,
        extensions: &[FileExtension::Dart],
        categories: &[Compiled, Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dart3,
            syntax: concat!(
                r"void main() {
    print('",
                output_for_hello_world!(),
                r"');
}"
            ),
            tokenized_syntax: &[
                (TokenKind::Type, "void"),
                (TokenKind::Plain, " "),
                (TokenKind::Function, "main"),
                (TokenKind::Punctuation, "()"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "{"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Function, "print"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("'", output_for_hello_world!(), "'"),
                ),
                (TokenKind::Punctuation, ");"),
                (TokenKind::Plain, "\n"),
                (TokenKind::Punctuation, "}"),
            ],
        }],
    };
