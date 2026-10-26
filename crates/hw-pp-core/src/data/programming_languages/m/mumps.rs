//! Hello-world snippet for `ProgrammingLanguageName::Mumps`.
//!
//! Imported from leachim6/hello-world (`m/Mumps.m` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mumps.m>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Historical, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mumps => "Mumps",
    versions: [Mumps => "Mumps"],
    extensions: [],
}

const MUMPS: &str = concat!(r#" w ""#, output_for_hello_world!(), r#"",!"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mumps,
        extensions: &[],
        categories: &[Historical],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mumps,
            syntax: MUMPS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MUMPS,
                    [(
                        TokenKind::Plain,
                        concat!(r#" w ""#, output_for_hello_world!(), r#"",!"#),
                    )],
                )
            },
        }],
    };
