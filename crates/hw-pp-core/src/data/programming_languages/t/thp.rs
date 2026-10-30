//! Hello-world snippet for `ProgrammingLanguageName::Thp`.
//!
//! Imported from leachim6/hello-world (`t/THP.hrp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/THP.hrp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Thp => "THP",
    versions: [Thp => "THP"],
    extensions: [Hrp => "hrp"],
}

const THP: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r#" in "The Harper Programmer"
; Compiled and tested with version 2.0 Sybex Edition

Variable        Msg,13,""#,
    output_for_hello_world!(),
    r#""

Write           Msg"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Thp,
        extensions: &[FileExtension::Hrp],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Thp,
            syntax: THP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    THP,
                    [(
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
                )
            },
        }],
    };
