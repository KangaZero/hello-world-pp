//! Hello-world snippet for `ProgrammingLanguageName::APlusPlus`.
//!
//! Imported from leachim6/hello-world (`a/A++` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/A%2B%2B>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: APlusPlus => "A++",
    versions: [APlusPlus => "A++"],
    extensions: [],
}

const A_PLUS_PLUS: &str = concat!(r#"(print ""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::APlusPlus,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::APlusPlus,
            syntax: A_PLUS_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    A_PLUS_PLUS,
                    [(
                        TokenKind::Plain,
                        concat!(r#"(print ""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
