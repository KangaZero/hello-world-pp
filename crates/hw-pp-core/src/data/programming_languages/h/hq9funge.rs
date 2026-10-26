//! Hello-world snippet for `ProgrammingLanguageName::Hq9funge`.
//!
//! Imported from leachim6/hello-world (`h/HQ9funge` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/HQ9funge>
//! - <https://esolangs.org/wiki/HQ9funge> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Reflective;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Hq9funge => "HQ9funge",
    versions: [Hq9funge => "HQ9funge"],
    extensions: [],
}

const HQ9FUNGE: &str = r"H";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hq9funge,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hq9funge,
            syntax: HQ9FUNGE,
            tokenized_syntax: &const { checked_token_lengths(HQ9FUNGE, [(TokenKind::Plain, r"H")]) },
        }],
    };
