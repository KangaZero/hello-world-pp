//! Hello-world snippet for `ProgrammingLanguageName::Emotinomicon`.
//!
//! Imported from leachim6/hello-world (`e/Emotinomicon` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Emotinomicon,
            syntax: r"😭dlroW olleH😲⏪⏬⏩",
            tokenized_syntax: &[(TokenKind::Plain, r"😭dlroW olleH😲⏪⏬⏩")],
        }],
    };
