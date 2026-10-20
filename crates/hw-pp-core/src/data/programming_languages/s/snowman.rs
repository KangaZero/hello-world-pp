//! Hello-world snippet for `ProgrammingLanguageName::Snowman`.
//!
//! Imported from leachim6/hello-world (`s/Snowman.sm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Snowman.sm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Snowman => "Snowman",
    versions: [Snowman => "Snowman"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Snowman,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Snowman,
            syntax: concat!(r#"~""#, output_for_hello_world!(), r#""sPvG"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"~""#, output_for_hello_world!(), r#""sPvG"#),
            )],
        }],
    };
