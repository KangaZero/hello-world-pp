//! Hello-world snippet for `ProgrammingLanguageName::Ibm1401`.
//!
//! Imported from leachim6/hello-world (`i/IBM1401` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/IBM1401>

use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::{Assembly, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Ibm1401 => "IBM1401",
    versions: [Ibm1401 => "IBM1401"],
    extensions: [],
}

const IBM1401: &str = r",008015,022029,036043,050054,055062,063065,066077/333/M0762502F1.HELLO
WORLD";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ibm1401,
        extensions: &[],
        categories: &[Assembly, Historical],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ibm1401,
            syntax: IBM1401,
            tokenized_syntax: &const {
                checked_token_lengths(
                    IBM1401,
                    [(
                        TokenKind::Plain,
                        r",008015,022029,036043,050054,055062,063065,066077/333/M0762502F1.HELLO
WORLD",
                    )],
                )
            },
        }],
    };
