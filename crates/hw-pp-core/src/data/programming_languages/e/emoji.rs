//! Hello-world snippet for `ProgrammingLanguageName::Emoji`.
//!
//! Imported from leachim6/hello-world (`e/Emoji` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Emoji>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Emoji => "Emoji",
    versions: [Emoji => "Emoji"],
    extensions: [],
}

const EMOJI: &str = concat!(r"💬", output_for_hello_world!(), r"💬➡");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Emoji,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Emoji,
            syntax: EMOJI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EMOJI,
                    [(
                        TokenKind::Plain,
                        concat!(r"💬", output_for_hello_world!(), r"💬➡"),
                    )],
                )
            },
        }],
    };
