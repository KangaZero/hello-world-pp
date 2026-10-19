//! Hello-world snippet for `ProgrammingLanguageName::Cpp2`.
//!
//! Imported from leachim6/hello-world (PR #1572 `c/c++2.cpp2`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1572>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cpp2 => "Cpp2",
    versions: [Cpp2 => "Cpp2"],
    extensions: [Cpp2 => "cpp2"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cpp2,
        extensions: &[FileExtension::Cpp2],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cpp2,
            syntax: concat!(
                r#"main: () = std::println(""#,
                output_for_hello_world!(),
                r#"");"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"main: () = std::println(""#,
                    output_for_hello_world!(),
                    r#"");"#
                ),
            )],
        }],
    };
