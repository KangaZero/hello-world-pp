//! Hello-world snippet for `ProgrammingLanguageName::Swift`.
//!
//! Sources:
//! - <https://docs.swift.org/swift-book/documentation/the-swift-programming-language/guidedtour/>
//! - <https://www.swift.org/getting-started/cli-swiftpm/>
//! - <https://www.wikidata.org/wiki/Q17118377> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Declarative, Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Swift => "Swift",
    versions: [Swift5 => "Swift 5"],
    extensions: [Swift => "swift"],
}

const SWIFT5: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Swift,
        extensions: &[FileExtension::Swift],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Imperative, ObjectOriented, Functional, Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Swift5,
            syntax: SWIFT5,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SWIFT5,
                    [
                        (TokenKind::Function, "print"),
                        (TokenKind::Punctuation, "("),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                        (TokenKind::Punctuation, ")"),
                    ],
                )
            },
        }],
    };
