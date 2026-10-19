//! Hello-world snippet for `ProgrammingLanguageName::Hello`.
//!
//! Imported from leachim6/hello-world (`h/Hello` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Hello => "Hello",
    versions: [Hello => "Hello"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hello,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hello,
            syntax: r"h",
            tokenized_syntax: &[(TokenKind::Plain, r"h")],
        }],
    };
