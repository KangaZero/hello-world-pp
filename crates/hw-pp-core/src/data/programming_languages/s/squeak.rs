//! Hello-world snippet for `ProgrammingLanguageName::Squeak`.
//!
//! Imported from the Hello World Collection (`files/Squeak.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Squeak.htm>
//! - <http://helloworldcollection.de/#Squeak>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Smalltalk,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Squeak => "Squeak",
    versions: [Squeak => "Squeak"],
    extensions: [],
}

const SQUEAK: &str = concat!(
    r#"""#,
    output_for_hello_world!(),
    r#" in Squeak"

Transcript show: '"#,
    output_for_hello_world!(),
    r"'"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Squeak,
        extensions: &[],
        categories: &[],
        families: &[Smalltalk],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Squeak,
            syntax: SQUEAK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SQUEAK,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r#"""#, output_for_hello_world!(), r#" in Squeak""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"Transcript"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"show"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                    ],
                )
            },
        }],
    };
