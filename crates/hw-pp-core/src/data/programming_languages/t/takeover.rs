//! Hello-world snippet for `ProgrammingLanguageName::Takeover`.
//!
//! Imported from leachim6/hello-world (`t/Takeover` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Takeover>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Takeover => "Takeover",
    versions: [Takeover => "Takeover"],
    extensions: [],
}

const TAKEOVER: &str = concat!(r"[", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Takeover,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Takeover,
            syntax: TAKEOVER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TAKEOVER,
                    [(TokenKind::Plain, concat!(r"[", output_for_hello_world!()))],
                )
            },
        }],
    };
