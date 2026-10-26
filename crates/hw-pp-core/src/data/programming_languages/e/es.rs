//! Hello-world snippet for `ProgrammingLanguageName::Es`.
//!
//! Imported from leachim6/hello-world (`e/es` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/es>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Es => "es",
    versions: [Es => "es"],
    extensions: [],
}

const ES: &str = concat!(r"echo ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Es,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Es,
            syntax: ES,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ES,
                    [
                        (TokenKind::Keyword, r"echo"),
                        (TokenKind::Plain, concat!(r" ", output_for_hello_world!())),
                    ],
                )
            },
        }],
    };
