//! Hello-world snippet for `ProgrammingLanguageName::Echolisp`.
//!
//! Imported from leachim6/hello-world (`e/EchoLisp.echolisp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/EchoLisp.echolisp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Echolisp => "EchoLisp",
    versions: [Echolisp => "EchoLisp"],
    extensions: [Echolisp => "echolisp"],
}

const ECHOLISP: &str = concat!(
    r#"(display ""#,
    output_for_hello_world!(),
    r#"" "color:blue")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Echolisp,
        extensions: &[FileExtension::Echolisp],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Echolisp,
            syntax: ECHOLISP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ECHOLISP,
                    [
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"display"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""color:blue""#),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
