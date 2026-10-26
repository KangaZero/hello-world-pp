//! Hello-world snippet for `ProgrammingLanguageName::Emotinomicon`.
//!
//! Imported from leachim6/hello-world (`e/Emotinomicon` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Emotinomicon>
//! - <https://esolangs.org/wiki/Emotinomicon> (paradigms)

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
    language: Emotinomicon => "Emotinomicon",
    versions: [Emotinomicon => "Emotinomicon"],
    extensions: [],
}

const EMOTINOMICON: &str = r"😭dlroW olleH😲⏪⏬⏩";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Emotinomicon,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Emotinomicon,
            syntax: EMOTINOMICON,
            tokenized_syntax: &const {
                checked_token_lengths(EMOTINOMICON, [(TokenKind::Plain, r"😭dlroW olleH😲⏪⏬⏩")])
            },
        }],
    };
