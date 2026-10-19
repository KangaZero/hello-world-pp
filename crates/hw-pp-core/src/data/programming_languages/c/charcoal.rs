//! Hello-world snippet for `ProgrammingLanguageName::Charcoal`.
//!
//! Imported from leachim6/hello-world (`c/Charcoal.cl` at `a152253`),
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
    language: Charcoal => "Charcoal",
    versions: [Charcoal => "Charcoal"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Charcoal,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Charcoal,
            syntax: output_for_hello_world!(),
            tokenized_syntax: &[(TokenKind::Plain, output_for_hello_world!())],
        }],
    };
