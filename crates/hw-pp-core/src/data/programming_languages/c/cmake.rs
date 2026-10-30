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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Cmake => "CMake",
    versions: [Cmake => "CMake"],
    extensions: [Cmake => "cmake"],
}

const CMAKE: &str = concat!(r#"message(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cmake,
        extensions: &[FileExtension::Cmake],
        categories: &[Configuration],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cmake,
            syntax: CMAKE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CMAKE,
                    [
                        (TokenKind::Keyword, r"message"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
