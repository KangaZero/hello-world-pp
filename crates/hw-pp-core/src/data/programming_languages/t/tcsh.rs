//! Hello-world snippet for `ProgrammingLanguageName::Tcsh`.
//!
//! Imported from leachim6/hello-world (`t/TCSH.tcsh` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Tcsh => "TCSH",
    versions: [Tcsh => "TCSH"],
    extensions: [Tcsh => "tcsh"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tcsh,
        extensions: &[FileExtension::Tcsh],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tcsh,
            syntax: concat!(
                r#"#!/bin/tcsh
echo ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#!/bin/tcsh
echo ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
