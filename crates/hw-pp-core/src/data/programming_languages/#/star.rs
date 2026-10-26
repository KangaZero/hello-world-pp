//! Hello-world snippet for `ProgrammingLanguageName::Star`.
//!
//! Imported from leachim6/hello-world (`#/∗` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%E2%88%97>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Star => "∗",
    versions: [Star => "∗"],
    extensions: [],
}

const STAR: &str = r"*";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Star,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Star,
            syntax: STAR,
            tokenized_syntax: &const { checked_token_lengths(STAR, [(TokenKind::Plain, r"*")]) },
        }],
    };
