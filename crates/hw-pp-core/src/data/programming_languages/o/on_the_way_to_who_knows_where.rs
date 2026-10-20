//! Hello-world snippet for `ProgrammingLanguageName::OnTheWayToWhoKnowsWhere`.
//!
//! Imported from leachim6/hello-world (`o/ON THE WAY TO WHO KNOWS WHERE` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/ON%20THE%20WAY%20TO%20WHO%20KNOWS%20WHERE>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: OnTheWayToWhoKnowsWhere => "ON THE WAY TO WHO KNOWS WHERE",
    versions: [OnTheWayToWhoKnowsWhere => "ON THE WAY TO WHO KNOWS WHERE"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OnTheWayToWhoKnowsWhere,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OnTheWayToWhoKnowsWhere,
            syntax: r"11",
            tokenized_syntax: &[(TokenKind::Plain, r"11")],
        }],
    };
