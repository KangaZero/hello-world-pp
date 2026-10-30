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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Snowman => "Snowman",
    versions: [Snowman => "Snowman"],
    extensions: [],
}

const SNOWMAN: &str = concat!(r#"~""#, output_for_hello_world!(), r#""sPvG"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Snowman,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Snowman,
            syntax: SNOWMAN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SNOWMAN,
                    [(
                        TokenKind::Plain,
                        concat!(r#"~""#, output_for_hello_world!(), r#""sPvG"#),
                    )],
                )
            },
        }],
    };
