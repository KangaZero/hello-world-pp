//! Hello-world snippet for `ProgrammingLanguageName::Carp`.
//!
//! Imported from the Hello World Collection (`files/Carp.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Carp.htm>
//! - <http://helloworldcollection.de/#Carp>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Carp => "Carp",
    versions: [Carp => "Carp"],
    extensions: [],
}

const CARP: &str = concat!(
    r";; ",
    output_for_hello_world!(),
    r#" in Carp

(println ""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Carp,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Carp,
            syntax: CARP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CARP,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r";; ", output_for_hello_world!(), r" in Carp"),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"println"),
                        (TokenKind::Plain, r" "),
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
