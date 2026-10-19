//! Hello-world snippet for `ProgrammingLanguageName::N05ab1e`.
//!
//! Imported from leachim6/hello-world (`#/05AB1E` at `a152253`),
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
    language: N05ab1e => "05AB1E",
    versions: [N05ab1e => "05AB1E"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N05ab1e,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N05ab1e,
            syntax: concat!(r#"""#, output_for_hello_world!()),
            tokenized_syntax: &[(TokenKind::Plain, concat!(r#"""#, output_for_hello_world!()))],
        }],
    };
