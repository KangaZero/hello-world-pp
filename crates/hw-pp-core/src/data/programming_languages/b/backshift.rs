//! Hello-world snippet for `ProgrammingLanguageName::Backshift`.
//!
//! Imported from leachim6/hello-world (`b/Backshift` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Backshift => "Backshift",
    versions: [Backshift => "Backshift"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Backshift,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Backshift,
            syntax: concat!(r" ", output_for_hello_world!()),
            tokenized_syntax: &[(TokenKind::Plain, concat!(r" ", output_for_hello_world!()))],
        }],
    };
