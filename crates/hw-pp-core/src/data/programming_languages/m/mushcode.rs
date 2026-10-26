//! Hello-world snippet for `ProgrammingLanguageName::Mushcode`.
//!
//! Imported from leachim6/hello-world (`m/Mushcode.mush` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mushcode.mush>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mushcode => "Mushcode",
    versions: [Mushcode => "Mushcode"],
    extensions: [Mush => "mush"],
}

const MUSHCODE: &str = concat!(r"say ", output_for_hello_world!(), r".");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mushcode,
        extensions: &[FileExtension::Mush],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mushcode,
            syntax: MUSHCODE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MUSHCODE,
                    [(
                        TokenKind::Plain,
                        concat!(r"say ", output_for_hello_world!(), r"."),
                    )],
                )
            },
        }],
    };
