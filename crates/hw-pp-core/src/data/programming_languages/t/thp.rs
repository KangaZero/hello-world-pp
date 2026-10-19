//! Hello-world snippet for `ProgrammingLanguageName::Thp`.
//!
//! Imported from leachim6/hello-world (`t/THP.hrp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Thp => "THP",
    versions: [Thp => "THP"],
    extensions: [Hrp => "hrp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Thp,
        extensions: &[FileExtension::Hrp],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Thp,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r#" in "The Harper Programmer"
; Compiled and tested with version 2.0 Sybex Edition

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
                    r#" in "The Harper Programmer"
; Compiled and tested with version 2.0 Sybex Edition

Variable        Msg,13,""#,
                    output_for_hello_world!(),
                    r#""

Write           Msg"#
                ),
            )],
        }],
    };
