//! Hello-world snippet for `ProgrammingLanguageName::Rainlisp`.
//!
//! Imported from leachim6/hello-world (PR #1561 `r/RainLisp.rl`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1561>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rainlisp => "RainLisp",
    versions: [Rainlisp => "RainLisp"],
    extensions: [Rl => "rl"],
}

const RAINLISP: &str = concat!(r#"(display ""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rainlisp,
        extensions: &[FileExtension::Rl],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rainlisp,
            syntax: RAINLISP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RAINLISP,
                    [
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"display"),
                        (TokenKind::Plain, r" "),
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
