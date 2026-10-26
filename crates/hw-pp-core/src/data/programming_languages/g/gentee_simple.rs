//! Hello-world snippet for `ProgrammingLanguageName::GenteeSimple`.
//!
//! Imported from the Hello World Collection (`files/Gentee-simple.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Gentee-simple.txt>
//! - <http://helloworldcollection.de/#Gentee-simple>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: GenteeSimple => "Gentee-simple",
    versions: [GenteeSimple => "Gentee-simple"],
    extensions: [],
}

const GENTEE_SIMPLE: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in Gentee (simple version)

func hello<main> : @""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GenteeSimple,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GenteeSimple,
            syntax: GENTEE_SIMPLE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GENTEE_SIMPLE,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r"// ",
                                output_for_hello_world!(),
                                r" in Gentee (simple version)"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"func"),
                        (TokenKind::Plain, r" hello<main> "),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" @"),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
