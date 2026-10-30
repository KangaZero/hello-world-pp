//! Hello-world snippet for `ProgrammingLanguageName::Hare`.
//!
//! Imported from leachim6/hello-world (`h/Hare.ha` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Hare.ha>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Hare => "Hare",
    versions: [Hare => "Hare"],
    extensions: [Ha => "ha"],
}

const HARE: &str = concat!(
    r#"use fmt;

export fn main() void = {
	fmt::println(""#,
    output_for_hello_world!(),
    r#"")!;
};"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hare,
        extensions: &[FileExtension::Ha],
        categories: &[Compiled],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hare,
            syntax: HARE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HARE,
                    [
                        (TokenKind::Keyword, r"use"),
                        (TokenKind::Plain, r" fmt"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"export"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"fn"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"void"),
                        (TokenKind::Plain, r" = "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
	fmt",
                        ),
                        (TokenKind::Punctuation, r"::"),
                        (TokenKind::Function, r"println"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r"!"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"};"),
                    ],
                )
            },
        }],
    };
