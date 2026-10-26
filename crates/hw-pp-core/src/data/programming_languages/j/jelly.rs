//! Hello-world snippet for `ProgrammingLanguageName::Jelly`.
//!
//! Imported from leachim6/hello-world (`j/Jelly.j` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/Jelly.j>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Jelly => "Jelly",
    versions: [Jelly => "Jelly"],
    extensions: [],
}

const JELLY: &str = r"“½,⁾ẇṭ»";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jelly,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jelly,
            syntax: JELLY,
            tokenized_syntax: &const {
                checked_token_lengths(JELLY, [(TokenKind::Plain, r"“½,⁾ẇṭ»")])
            },
        }],
    };
