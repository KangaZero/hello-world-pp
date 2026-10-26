//! Hello-world snippet for `ProgrammingLanguageName::CStar`.
//!
//! Imported from leachim6/hello-world (`c/C∗` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/C%E2%88%97>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::C,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: CStar => "C∗",
    versions: [CStar => "C∗"],
    extensions: [],
}

const C_STAR: &str = concat!(
    r#"#include <stdio.h>

main()
{
    printf(""#,
    output_for_hello_world!(),
    r#"\n");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CStar,
        extensions: &[],
        categories: &[],
        families: &[C],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CStar,
            syntax: C_STAR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    C_STAR,
                    [
                        (TokenKind::Macro, r"#include <stdio.h>"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
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
    ",
                        ),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
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
