//! Hello-world snippet for `ProgrammingLanguageName::U1f192`.
//!
//! Imported from leachim6/hello-world (`#/🆒` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%F0%9F%86%92>
//! - <https://esolangs.org/wiki/%F0%9F%86%92> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: U1f192 => "🆒",
    versions: [U1f192 => "🆒"],
    extensions: [],
}

const U1F192: &str = r"💬🔤👋🗺️🔤";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U1f192,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U1f192,
            syntax: U1F192,
            tokenized_syntax: &const {
                checked_token_lengths(U1F192, [(TokenKind::Plain, r"💬🔤👋🗺️🔤")])
            },
        }],
    };
