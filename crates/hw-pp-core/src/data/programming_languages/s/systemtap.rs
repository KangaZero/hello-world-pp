//! Hello-world snippet for `ProgrammingLanguageName::Systemtap`.
//!
//! Imported from leachim6/hello-world (`s/SystemTap.stp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SystemTap.stp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Systemtap => "SystemTap",
    versions: [Systemtap => "SystemTap"],
    extensions: [Stp => "stp"],
}

const SYSTEMTAP: &str = concat!(
    r#"probe oneshot { println(""#,
    output_for_hello_world!(),
    r#"") }"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Systemtap,
        extensions: &[FileExtension::Stp],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Systemtap,
            syntax: SYSTEMTAP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SYSTEMTAP,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"probe oneshot { println(""#,
                            output_for_hello_world!(),
                            r#"") }"#
                        ),
                    )],
                )
            },
        }],
    };
