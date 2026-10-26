//! Hello-world snippet for `ProgrammingLanguageName::Gentee`.
//!
//! Imported from the Hello World Collection (`files/Gentee.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Gentee.txt>
//! - <http://helloworldcollection.de/#Gentee>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Gentee => "Gentee",
    versions: [Gentee => "Gentee"],
    extensions: [],
}

const GENTEE: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in Gentee

func hello <main>
{
   print( ""#,
    output_for_hello_world!(),
    r#"" )
   getch()
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gentee,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gentee,
            syntax: GENTEE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GENTEE,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r"// ", output_for_hello_world!(), r" in Gentee"),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"func"),
                        (
                            TokenKind::Plain,
                            r" hello <main>
",
                        ),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"getch"),
                        (TokenKind::Punctuation, r"()"),
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
