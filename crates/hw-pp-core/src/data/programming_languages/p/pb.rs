//! Hello-world snippet for `ProgrammingLanguageName::Pb`.
//!
//! Imported from leachim6/hello-world (`p/PB.pwr` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PB.pwr>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Pb => "PB",
    versions: [Pb => "PB"],
    extensions: [Pwr => "pwr"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pb,
        extensions: &[FileExtension::Pwr],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pb,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r#" in "PowerBatch Plus+" from Computing Systems Design, Inc.

Variable        Msg,13,""#,
                output_for_hello_world!(),
                r#""

Write           Msg"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"; ",
                    output_for_hello_world!(),
                    r#" in "PowerBatch Plus+" from Computing Systems Design, Inc.

Variable        Msg,13,""#,
                    output_for_hello_world!(),
                    r#""

Write           Msg"#
                ),
            )],
        }],
    };
