//! Hello-world snippet for `ProgrammingLanguageName::Lost`.
//!
//! Imported from leachim6/hello-world (`l/Lost.？？？` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Lost.%EF%BC%9F%EF%BC%9F%EF%BC%9F>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Lost => "Lost",
    versions: [Lost => "Lost"],
    extensions: [Uff1fUff1fUff1f => "？？？"],
}

const LOST: &str = concat!(r#"%""#, output_for_hello_world!(), r#""@"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lost,
        extensions: &[FileExtension::Uff1fUff1fUff1f],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lost,
            syntax: LOST,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LOST,
                    [(
                        TokenKind::Plain,
                        concat!(r#"%""#, output_for_hello_world!(), r#""@"#),
                    )],
                )
            },
        }],
    };
