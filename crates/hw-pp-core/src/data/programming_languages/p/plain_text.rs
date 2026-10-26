//! Hello-world snippet for `ProgrammingLanguageName::PlainText`.
//!
//! Imported from leachim6/hello-world (`t/Plain text.txt` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Plain%20text.txt>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Markup, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: PlainText => "Plain text",
    versions: [PlainText => "Plain text"],
    extensions: [],
}

const PLAIN_TEXT: &str = output_for_hello_world!();

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PlainText,
        extensions: &[],
        categories: &[Markup],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PlainText,
            syntax: PLAIN_TEXT,
            tokenized_syntax: &const {
                checked_token_lengths(PLAIN_TEXT, [(TokenKind::Plain, output_for_hello_world!())])
            },
        }],
    };
