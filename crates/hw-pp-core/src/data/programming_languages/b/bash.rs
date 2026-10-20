//! Hello-world snippets for `ProgrammingLanguageName::Bash`.
//!
//! Sources:
//! - <https://pubs.opengroup.org/onlinepubs/9799919799/utilities/echo.html>
//! - <https://pubs.opengroup.org/onlinepubs/9799919799/utilities/V3_chap02.html>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Bash => "Bash",
    versions: [PosixShell => "POSIX shell"],
    extensions: [Sh => "sh", Bash => "bash"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bash,
        extensions: &[FileExtension::Sh, FileExtension::Bash],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PosixShell,
            syntax: concat!(r#"echo ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Function, "echo"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
            ],
        }],
    };
