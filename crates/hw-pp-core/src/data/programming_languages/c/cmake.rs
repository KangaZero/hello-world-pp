//! Hello-world snippet for `ProgrammingLanguageName::Cmake`.
//!
//! Imported from leachim6/hello-world (`c/CMake.cmake` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CMake.cmake>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Configuration,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cmake => "CMake",
    versions: [Cmake => "CMake"],
    extensions: [Cmake => "cmake"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cmake,
        extensions: &[FileExtension::Cmake],
        categories: &[Configuration],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cmake,
            syntax: concat!(r#"message(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"message"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
            ],
        }],
    };
