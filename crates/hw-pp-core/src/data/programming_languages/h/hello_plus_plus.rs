//! Hello-world snippet for `ProgrammingLanguageName::HelloPlusPlus`.
//!
//! Imported from leachim6/hello-world (`h/Hello++` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Hello%2B%2B>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: HelloPlusPlus => "Hello++",
    versions: [HelloPlusPlus => "Hello++"],
    extensions: [],
}

const HELLO_PLUS_PLUS: &str = r"h";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::HelloPlusPlus,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::HelloPlusPlus,
            syntax: HELLO_PLUS_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(HELLO_PLUS_PLUS, [(TokenKind::Plain, r"h")])
            },
        }],
    };
