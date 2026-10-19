//! Hello-world snippets for `ProgrammingLanguageName::Swift`.
//!
//! Sources:
//! - <https://docs.swift.org/swift-book/documentation/the-swift-programming-language/guidedtour/>
//! - <https://www.swift.org/getting-started/cli-swiftpm/>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Swift => "Swift",
    versions: [Swift5 => "Swift 5"],
    extensions: [Swift => "swift"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Swift,
        extensions: &[FileExtension::Swift],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Swift5,
            syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Function, "print"),
                (TokenKind::Punctuation, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ")"),
            ],
        }],
    };
