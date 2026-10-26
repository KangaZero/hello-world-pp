//! Hello-world snippet for `ProgrammingLanguageName::Awful`.
//!
//! Imported from leachim6/hello-world (`a/Awful.yuk` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Awful.yuk>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Awful => "Awful",
    versions: [Awful => "Awful"],
    extensions: [Yuk => "yuk"],
}

const AWFUL: &str = concat!(
    r#"#!/usr/bin/awful

:writeln s""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Awful,
        extensions: &[FileExtension::Yuk],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Awful,
            syntax: AWFUL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AWFUL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"#!/usr/bin/awful

:writeln s""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
