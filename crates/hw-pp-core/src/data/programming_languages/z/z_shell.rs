//! Hello-world snippet for `ProgrammingLanguageName::ZShell`.
//!
//! Imported from leachim6/hello-world (`z/Z Shell.zsh` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/Z%20Shell.zsh>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: ZShell => "Z Shell",
    versions: [ZShell => "Z Shell"],
    extensions: [Zsh => "zsh"],
}

const Z_SHELL: &str = concat!(
    r#"#!/bin/zsh
echo ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ZShell,
        extensions: &[FileExtension::Zsh],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ZShell,
            syntax: Z_SHELL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    Z_SHELL,
                    [
                        (
                            TokenKind::Comment,
                            r"#!/bin/zsh
",
                        ),
                        (TokenKind::Function, r"echo"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
