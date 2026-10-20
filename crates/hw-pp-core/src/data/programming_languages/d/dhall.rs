//! Hello-world snippet for `ProgrammingLanguageName::Dhall`.
//!
//! Imported from leachim6/hello-world (PR #1837 `d/Dhall.dhall`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1837>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Configuration,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dhall => "Dhall",
    versions: [Dhall => "Dhall"],
    extensions: [Dhall => "dhall"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dhall,
        extensions: &[FileExtension::Dhall],
        categories: &[Configuration],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dhall,
            syntax: concat!(
                r#"let hello = ""#,
                output_for_hello_world!(),
                r#""

in hello"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"let"),
                (TokenKind::Plain, r" hello = "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"in"),
                (TokenKind::Plain, r" hello"),
            ],
        }],
    };
