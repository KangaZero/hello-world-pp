//! Hello-world snippet for `ProgrammingLanguageName::Underload`.
//!
//! Imported from leachim6/hello-world (`u/Underload.ul` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/u/Underload.ul>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Underload => "Underload",
    versions: [Underload => "Underload"],
    extensions: [Ul => "ul"],
}

const UNDERLOAD: &str = concat!(r"(", output_for_hello_world!(), r")S");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Underload,
        extensions: &[FileExtension::Ul],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Underload,
            syntax: UNDERLOAD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    UNDERLOAD,
                    [(
                        TokenKind::Plain,
                        concat!(r"(", output_for_hello_world!(), r")S"),
                    )],
                )
            },
        }],
    };
