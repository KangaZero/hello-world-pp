//! Hello-world snippet for `ProgrammingLanguageName::C3`.
//!
//! Imported from leachim6/hello-world (PR #1598 `c/C3.c3`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1598>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: C3 => "C3",
    versions: [C3 => "C3"],
    extensions: [C3 => "c3"],
}

const C3: &str = concat!(
    r#"import std::io;

fn void main()
{
    io::printn(""#,
    output_for_hello_world!(),
    r#"");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::C3,
        extensions: &[FileExtension::C3],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::C3,
            syntax: C3,
            tokenized_syntax: &const {
                checked_token_lengths(
                    C3,
                    [
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" std"),
                        (TokenKind::Punctuation, r"::"),
                        (TokenKind::Plain, r"io"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"fn"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"void"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
    io",
                        ),
                        (TokenKind::Punctuation, r"::"),
                        (TokenKind::Function, r"printn"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
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
