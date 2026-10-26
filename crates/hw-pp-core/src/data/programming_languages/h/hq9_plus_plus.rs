//! Hello-world snippet for `ProgrammingLanguageName::Hq9PlusPlus`.
//!
//! Imported from leachim6/hello-world (`h/HQ9++` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/HQ9%2B%2B>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Hq9PlusPlus => "HQ9++",
    versions: [Hq9PlusPlus => "HQ9++"],
    extensions: [],
}

const HQ9_PLUS_PLUS: &str = r"H";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hq9PlusPlus,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hq9PlusPlus,
            syntax: HQ9_PLUS_PLUS,
            tokenized_syntax: &const { checked_token_lengths(HQ9_PLUS_PLUS, [(TokenKind::Plain, r"H")]) },
        }],
    };
