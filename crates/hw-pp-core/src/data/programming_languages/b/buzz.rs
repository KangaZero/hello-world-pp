//! Hello-world snippet for `ProgrammingLanguageName::Buzz`.
//!
//! Imported from the Hello World Collection (`files/Buzz.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Buzz.htm>
//! - <http://helloworldcollection.de/#Buzz>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Buzz => "Buzz",
    versions: [Buzz => "Buzz"],
    extensions: [],
}

const BUZZ: &str = concat!(
    r"# ",
    output_for_hello_world!(),
    r#" in Buzz

log(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Buzz,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Buzz,
            syntax: BUZZ,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BUZZ,
                    [
                        (
                            TokenKind::Macro,
                            concat!(r"# ", output_for_hello_world!(), r" in Buzz"),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Function, r"log"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
