//! Hello-world snippet for `ProgrammingLanguageName::N0NopCaret`.
//!
//! Imported from leachim6/hello-world (`#/0(nop^)` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/0%28nop%5E%29>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: N0NopCaret => "0(nop^)",
    versions: [N0NopCaret => "0(nop^)"],
    extensions: [],
}

const N0_NOP_CARET: &str = concat!(
    r"1$(",
    output_for_hello_world!(),
    r#"
")`"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N0NopCaret,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N0NopCaret,
            syntax: N0_NOP_CARET,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N0_NOP_CARET,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"1$(",
                            output_for_hello_world!(),
                            r#"
")`"#
                        ),
                    )],
                )
            },
        }],
    };
