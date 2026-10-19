//! Hello-world snippet for `ProgrammingLanguageName::YourMom`.
//!
//! Imported from leachim6/hello-world (`y/Your Mom` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/y/Your%20Mom>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: YourMom => "Your Mom",
    versions: [YourMom => "Your Mom"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::YourMom,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::YourMom,
            syntax: concat!(r"'", output_for_hello_world!()),
            tokenized_syntax: &[(TokenKind::Plain, concat!(r"'", output_for_hello_world!()))],
        }],
    };
