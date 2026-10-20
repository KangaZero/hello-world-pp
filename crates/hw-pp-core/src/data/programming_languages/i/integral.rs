//! Hello-world snippet for `ProgrammingLanguageName::Integral`.
//!
//! Imported from leachim6/hello-world (`i/Integral` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/Integral>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Integral => "Integral",
    versions: [Integral => "Integral"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Integral,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Integral,
            syntax: concat!(r"⌡", output_for_hello_world!()),
            tokenized_syntax: &[(TokenKind::Plain, concat!(r"⌡", output_for_hello_world!()))],
        }],
    };
