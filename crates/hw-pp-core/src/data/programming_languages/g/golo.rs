//! Hello-world snippet for `ProgrammingLanguageName::Golo`.
//!
//! Imported from leachim6/hello-world (`g/Golo.golo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Golo.golo>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Golo => "Golo",
    versions: [Golo => "Golo"],
    extensions: [Golo => "golo"],
}

const GOLO: &str = concat!(
    r#"module hello.world

function main = |args| {
  println(""#,
    output_for_hello_world!(),
    r#"")
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Golo,
        extensions: &[FileExtension::Golo],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Golo,
            syntax: GOLO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GOLO,
                    [
                        (TokenKind::Keyword, r"module"),
                        (TokenKind::Plain, r" hello"),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"world

",
                        ),
                        (TokenKind::Keyword, r"function"),
                        (TokenKind::Plain, r" main = |args| "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Function, r"println"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
