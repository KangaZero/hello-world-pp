//! Hello-world snippet for `ProgrammingLanguageName::Tcsh`.
//!
//! Imported from leachim6/hello-world (`t/TCSH.tcsh` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/TCSH.tcsh>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Tcsh => "TCSH",
    versions: [Tcsh => "TCSH"],
    extensions: [Tcsh => "tcsh"],
}

const TCSH: &str = concat!(
    r#"#!/bin/tcsh
echo ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tcsh,
        extensions: &[FileExtension::Tcsh],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tcsh,
            syntax: TCSH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TCSH,
                    [
                        (TokenKind::Comment, r"#!/bin/tcsh"),
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
