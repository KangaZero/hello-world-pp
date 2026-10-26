//! Hello-world snippet for `ProgrammingLanguageName::Terra`.
//!
//! Imported from leachim6/hello-world (`t/Terra.t` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Terra.t>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Terra => "Terra",
    versions: [Terra => "Terra"],
    extensions: [T => "t"],
}

const TERRA: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Terra,
        extensions: &[FileExtension::T],
        categories: &[Compiled],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Terra,
            syntax: TERRA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TERRA,
                    [
                        (TokenKind::Function, r"print"),
                        (TokenKind::Plain, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Plain, r")"),
                    ],
                )
            },
        }],
    };
