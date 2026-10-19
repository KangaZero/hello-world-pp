//! Hello-world snippet for `ProgrammingLanguageName::Emotinomicon`.
//!
//! Imported from leachim6/hello-world (`e/Emotinomicon` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Emotinomicon>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Emotinomicon => "Emotinomicon",
    versions: [Emotinomicon => "Emotinomicon"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Emotinomicon,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Emotinomicon,
            syntax: r"😭dlroW olleH😲⏪⏬⏩",
            tokenized_syntax: &[(TokenKind::Plain, r"😭dlroW olleH😲⏪⏬⏩")],
        }],
    };
