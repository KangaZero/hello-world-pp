//! Hello-world snippet for `ProgrammingLanguageName::OnTheWayToWhoKnowsWhere`.
//!
//! Imported from leachim6/hello-world (`o/ON THE WAY TO WHO KNOWS WHERE` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/ON%20THE%20WAY%20TO%20WHO%20KNOWS%20WHERE>
//! - <https://esolangs.org/wiki/ON_THE_WAY_TO_WHO_KNOWS_WHERE> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: OnTheWayToWhoKnowsWhere => "ON THE WAY TO WHO KNOWS WHERE",
    versions: [OnTheWayToWhoKnowsWhere => "ON THE WAY TO WHO KNOWS WHERE"],
    extensions: [],
}

const ON_THE_WAY_TO_WHO_KNOWS_WHERE: &str = r"11";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OnTheWayToWhoKnowsWhere,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OnTheWayToWhoKnowsWhere,
            syntax: ON_THE_WAY_TO_WHO_KNOWS_WHERE,
            tokenized_syntax: &const {
                checked_token_lengths(ON_THE_WAY_TO_WHO_KNOWS_WHERE, [(TokenKind::Plain, r"11")])
            },
        }],
    };
