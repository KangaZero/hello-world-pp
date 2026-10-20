//! Hello-world snippet for `ProgrammingLanguageName::Telephone`.
//!
//! Imported from leachim6/hello-world (`t/Telephone` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Telephone>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Telephone => "Telephone",
    versions: [Telephone => "Telephone"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Telephone,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Telephone,
            syntax: concat!(
                r"#How to Spell ",
                output_for_hello_world!(),
                r" on a standard telephone keypad
43556 96753"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"#How to Spell ",
                    output_for_hello_world!(),
                    r" on a standard telephone keypad
43556 96753"
                ),
            )],
        }],
    };
