//! Hello-world snippet for `ProgrammingLanguageName::Battlestar`.
//!
//! Imported from leachim6/hello-world (`b/Battlestar.bts` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Battlestar.bts>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Battlestar => "Battlestar",
    versions: [Battlestar => "Battlestar"],
    extensions: [Bts => "bts"],
}

const BATTLESTAR: &str = concat!(
    r#"#!/usr/bin/bts
const hello = ""#,
    output_for_hello_world!(),
    r#"\n"
print(hello)"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Battlestar,
        extensions: &[FileExtension::Bts],
        categories: &[Compiled],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Battlestar,
            syntax: BATTLESTAR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BATTLESTAR,
                    [
                        (TokenKind::Macro, r"#!/usr/bin/bts"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"const"),
                        (TokenKind::Plain, r" hello = "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"hello"),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
