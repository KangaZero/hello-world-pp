//! Hello-world snippet for `ProgrammingLanguageName::Sensetalk`.
//!
//! Imported from the Hello World Collection (`files/SenseTalk.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SenseTalk.txt>
//! - <http://helloworldcollection.de/#SenseTalk>
//! - <https://www.wikidata.org/wiki/Q7450966> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sensetalk => "SenseTalk",
    versions: [Sensetalk => "SenseTalk"],
    extensions: [],
}

const SENSETALK: &str = concat!(
    output_for_hello_world!(),
    r#" in SenseTalk.

on run put ""#,
    output_for_hello_world!(),
    r#"" end run "#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sensetalk,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sensetalk,
            syntax: SENSETALK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SENSETALK,
                    [
                        (TokenKind::Plain, concat!(output_for_hello_world!(), r" ")),
                        (TokenKind::Keyword, r"in"),
                        (TokenKind::Plain, r" SenseTalk"),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"on"),
                        (TokenKind::Plain, r" run "),
                        (TokenKind::Keyword, r"put"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"end"),
                        (TokenKind::Plain, r" run "),
                    ],
                )
            },
        }],
    };
