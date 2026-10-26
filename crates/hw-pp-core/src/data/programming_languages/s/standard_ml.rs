//! Hello-world snippet for `ProgrammingLanguageName::StandardMl`.
//!
//! Imported from leachim6/hello-world (`s/Standard ML.sml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Standard%20ML.sml>
//! - <https://www.wikidata.org/wiki/Q597330> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Imperative, Procedural};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Ml, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: StandardMl => "Standard ML",
    versions: [StandardMl => "Standard ML"],
    extensions: [],
}

const STANDARD_ML: &str = concat!(
    r#"fun hello() = print(""#,
    output_for_hello_world!(),
    r#"\n");

hello()"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::StandardMl,
        extensions: &[],
        categories: &[Compiled],
        families: &[Ml],
        paradigms: &[Imperative, Procedural, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::StandardMl,
            syntax: STANDARD_ML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    STANDARD_ML,
                    [
                        (TokenKind::Keyword, r"fun"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"hello"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" = "),
                        (TokenKind::Keyword, r"print"),
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
                        (TokenKind::Function, r"hello"),
                        (TokenKind::Punctuation, r"()"),
                    ],
                )
            },
        }],
    };
