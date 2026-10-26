//! Hello-world snippet for `ProgrammingLanguageName::Goose`.
//!
//! From aardappel/goose (`bench/bddicken/hello-world/goose/run.goose` at `a108677`),
//! Apache-2.0 licensed.
//!
//! Sources:
//! - <https://github.com/aardappel/goose/blob/a108677d300cb0e8bbb842c35a353abe67a20c6c/bench/bddicken/hello-world/goose/run.goose>
//! - <https://github.com/aardappel/goose>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Goose => "Goose",
    versions: [Goose => "Goose"],
    extensions: [Goose => "goose"],
}

const GOOSE: &str = concat!(
    r#"fn main() {
    print(""#,
    output_for_hello_world!(),
    r#"");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Goose,
        extensions: &[FileExtension::Goose],
        categories: &[Compiled],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Goose,
            syntax: GOOSE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GOOSE,
                    [
                        (TokenKind::Keyword, "fn"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Function, "main"),
                        (TokenKind::Punctuation, "()"),
                        (TokenKind::Plain, " "),
                        (TokenKind::Punctuation, "{"),
                        (TokenKind::Plain, "\n    "),
                        (TokenKind::Function, "print"),
                        (TokenKind::Punctuation, "("),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                        (TokenKind::Punctuation, ");"),
                        (TokenKind::Plain, "\n"),
                        (TokenKind::Punctuation, "}"),
                    ],
                )
            },
        }],
    };
