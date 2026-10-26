//! Hello-world snippet for `ProgrammingLanguageName::Batch`.
//!
//! Imported from leachim6/hello-world (`b/Batch.bat` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Batch.bat>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Batch => "Batch",
    versions: [Batch => "Batch"],
    extensions: [Bat => "bat"],
}

const BATCH: &str = concat!(
    r"@echo off
echo ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Batch,
        extensions: &[FileExtension::Bat],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Batch,
            syntax: BATCH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BATCH,
                    [
                        (TokenKind::Keyword, r"@echo"),
                        (
                            TokenKind::Plain,
                            r" off
",
                        ),
                        (TokenKind::Keyword, r"echo"),
                        (TokenKind::Plain, concat!(r" ", output_for_hello_world!())),
                    ],
                )
            },
        }],
    };
