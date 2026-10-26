//! Hello-world snippet for `ProgrammingLanguageName::CShell`.
//!
//! Imported from leachim6/hello-world (`c/C Shell.csh` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/C%20Shell.csh>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: CShell => "C Shell",
    versions: [CShell => "C Shell"],
    extensions: [Csh => "csh"],
}

const C_SHELL: &str = concat!(
    r#"#!/bin/csh
echo ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CShell,
        extensions: &[FileExtension::Csh],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CShell,
            syntax: C_SHELL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    C_SHELL,
                    [
                        (TokenKind::Comment, r"#!/bin/csh"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"echo"),
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
