//! Hello-world snippet for `ProgrammingLanguageName::Bash`.
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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Bash => "Bash",
    versions: [PosixShell => "POSIX shell"],
    extensions: [Sh => "sh", Bash => "bash"],
}

const POSIX_SHELL: &str = concat!(r#"echo ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bash,
        extensions: &[FileExtension::Sh, FileExtension::Bash],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PosixShell,
            syntax: POSIX_SHELL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    POSIX_SHELL,
                    [
                        (TokenKind::Function, "echo"),
                        (TokenKind::Plain, " "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "\""),
                        ),
                    ],
                )
            },
        }],
    };
