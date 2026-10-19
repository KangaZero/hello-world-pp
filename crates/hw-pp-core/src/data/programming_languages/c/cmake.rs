//! Hello-world snippet for `ProgrammingLanguageName::Cmake`.
//!
//! Imported from leachim6/hello-world (`c/CMake.cmake` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cmake,
            syntax: concat!(r#"message(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"message(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
