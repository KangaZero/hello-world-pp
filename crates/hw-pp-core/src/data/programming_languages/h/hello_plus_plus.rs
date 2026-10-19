//! Hello-world snippet for `ProgrammingLanguageName::HelloPlusPlus`.
//!
//! Imported from leachim6/hello-world (`h/Hello++` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: HelloPlusPlus => "Hello++",
    versions: [HelloPlusPlus => "Hello++"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::HelloPlusPlus,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::HelloPlusPlus,
            syntax: r"h",
            tokenized_syntax: &[(TokenKind::Plain, r"h")],
        }],
    };
