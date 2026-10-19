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
    programming_languages_token::TokenKind,
};

language_names! {
    language: ZShell => "Z Shell",
    versions: [ZShell => "Z Shell"],
    extensions: [Zsh => "zsh"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ZShell,
        extensions: &[FileExtension::Zsh],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ZShell,
            syntax: concat!(
                r#"#!/bin/zsh
echo ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
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
        }],
    };
