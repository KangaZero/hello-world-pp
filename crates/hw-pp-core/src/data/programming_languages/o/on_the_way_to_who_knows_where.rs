//! Hello-world snippet for `ProgrammingLanguageName::OnTheWayToWhoKnowsWhere`.
//!
//! Imported from leachim6/hello-world (`o/ON THE WAY TO WHO KNOWS WHERE` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OnTheWayToWhoKnowsWhere,
            syntax: r"11",
            tokenized_syntax: &[(TokenKind::Plain, r"11")],
        }],
    };
