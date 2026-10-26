//! Hello-world snippet for `ProgrammingLanguageName::RegularExpression`.
//!
//! Imported from the Hello World Collection (`files/Regular&nbsp;Expression.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Regular%26nbsp%3BExpression.txt>
//! - <http://helloworldcollection.de/#Regular%C2%A0Expression>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: RegularExpression => "Regular Expression",
    versions: [RegularExpression => "Regular Expression"],
    extensions: [],
}

const REGULAR_EXPRESSION: &str = concat!(
    output_for_hello_world!(),
    r#" as a regular expression.
Replaces everything with ""#,
    output_for_hello_world!(),
    r#"".
For use with vi, sed, etc.

Search String :  ^.*$
Replace String: '"#,
    output_for_hello_world!(),
    r"'"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::RegularExpression,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::RegularExpression,
            syntax: REGULAR_EXPRESSION,
            tokenized_syntax: &const {
                checked_token_lengths(
                    REGULAR_EXPRESSION,
                    [
                        (
                            TokenKind::Plain,
                            concat!(output_for_hello_world!(), r" as a regular expression"),
                        ),
                        (TokenKind::Keyword, r"."),
                        (
                            TokenKind::Plain,
                            concat!(
                                r#"
Replaces everything with ""#,
                                output_for_hello_world!(),
                                r#"""#
                            ),
                        ),
                        (TokenKind::Keyword, r"."),
                        (
                            TokenKind::Plain,
                            r"
For use with vi, sed, etc",
                        ),
                        (TokenKind::Keyword, r"."),
                        (
                            TokenKind::Plain,
                            r"

Search String :  ",
                        ),
                        (TokenKind::Keyword, r"^.*$"),
                        (
                            TokenKind::Plain,
                            concat!(
                                r"
Replace String: '",
                                output_for_hello_world!(),
                                r"'"
                            ),
                        ),
                    ],
                )
            },
        }],
    };
