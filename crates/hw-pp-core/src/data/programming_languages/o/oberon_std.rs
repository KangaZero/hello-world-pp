//! Hello-world snippet for `ProgrammingLanguageName::OberonStd`.
//!
//! Imported from the Hello World Collection (`files/Oberon.std` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Oberon.std>
//! - <http://helloworldcollection.de/#Oberon.std>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: OberonStd => "Oberon.std",
    versions: [OberonStd => "Oberon.std"],
    extensions: [],
}

const OBERON_STD: &str = concat!(
    r"(* ",
    output_for_hello_world!(),
    r#" in Oberon for standard operating systems *)

MODULE HelloWorld;
IMPORT Out;
BEGIN
  Out.String(""#,
    output_for_hello_world!(),
    r#"");
  Out.Ln;
END HelloWorld;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OberonStd,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OberonStd,
            syntax: OBERON_STD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OBERON_STD,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r"(* ",
                                output_for_hello_world!(),
                                r" in Oberon for standard operating systems *)"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"MODULE"),
                        (TokenKind::Plain, r" HelloWorld"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"IMPORT"),
                        (TokenKind::Plain, r" Out"),
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
  Out",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"String"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
  Out",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"Ln"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"END"),
                        (TokenKind::Plain, r" HelloWorld"),
                        (TokenKind::Punctuation, r";"),
                    ],
                )
            },
        }],
    };
