//! Hello-world snippet for `ProgrammingLanguageName::Jolie`.
//!
//! Imported from leachim6/hello-world (`j/Jolie.ol` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/Jolie.ol>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Jolie => "Jolie",
    versions: [Jolie => "Jolie"],
    extensions: [Ol => "ol"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jolie,
        extensions: &[FileExtension::Ol],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jolie,
            syntax: concat!(
                r#"include "console.iol"

main {
	println@Console(""#,
                output_for_hello_world!(),
                r#"")()
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, r"include "),
                (TokenKind::String, r#""console.iol""#),
                (
                    TokenKind::Plain,
                    r"

main ",
                ),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Function, r"println"),
                (TokenKind::Macro, r"@Console"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")()"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };
