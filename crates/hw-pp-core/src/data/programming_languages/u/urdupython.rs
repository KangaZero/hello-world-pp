//! Hello-world snippet for `ProgrammingLanguageName::Urdupython`.
//!
//! Imported from leachim6/hello-world (`u/UrduPython.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/u/UrduPython.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Urdupython => "UrduPython",
    versions: [Urdupython => "UrduPython"],
    extensions: [],
}

const URDUPYTHON: &str = concat!(r#"لکھو (""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Urdupython,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Urdupython,
            syntax: URDUPYTHON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    URDUPYTHON,
                    [(
                        TokenKind::Plain,
                        concat!(r#"لکھو (""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
