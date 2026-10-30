//! Hello-world snippet for `ProgrammingLanguageName::Fist`.
//!
//! Imported from leachim6/hello-world (`f/Fist` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fist>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Fist => "Fist",
    versions: [Fist => "Fist"],
    extensions: [],
}

const FIST: &str = concat!(r".", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fist,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fist,
            syntax: FIST,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FIST,
                    [(TokenKind::Plain, concat!(r".", output_for_hello_world!()))],
                )
            },
        }],
    };
