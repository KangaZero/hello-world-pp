//! Hello-world snippet for `ProgrammingLanguageName::Modula3`.
//!
//! Imported from leachim6/hello-world (`m/Modula 3.m3` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Modula%203.m3>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Modula,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Modula3 => "Modula 3",
    versions: [Modula3 => "Modula 3"],
    extensions: [M3 => "m3"],
}

const MODULA3: &str = concat!(
    r#"MODULE HelloWorld;
IMPORT Io;
BEGIN
    IO.Put (""#,
    output_for_hello_world!(),
    r#"\n")
END HelloWorld."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Modula3,
        extensions: &[FileExtension::M3],
        categories: &[Compiled, Historical],
        families: &[Modula],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Modula3,
            syntax: MODULA3,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MODULA3,
                    [
                        (TokenKind::Keyword, r"MODULE"),
                        (TokenKind::Plain, r" HelloWorld"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"IMPORT"),
                        (TokenKind::Plain, r" Io"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"BEGIN"),
                        (
                            TokenKind::Plain,
                            r"
    IO",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"Put"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"END"),
                        (TokenKind::Plain, r" HelloWorld"),
                        (TokenKind::Punctuation, r"."),
                    ],
                )
            },
        }],
    };
