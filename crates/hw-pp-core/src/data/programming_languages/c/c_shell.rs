//! Hello-world snippet for `ProgrammingLanguageName::CShell`.
//!
//! Imported from leachim6/hello-world (`c/C Shell.csh` at `a152253`),
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
    language: CShell => "C Shell",
    versions: [CShell => "C Shell"],
    extensions: [Csh => "csh"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CShell,
        extensions: &[FileExtension::Csh],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CShell,
            syntax: concat!(
                r#"#!/bin/csh
echo ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#!/bin/csh
echo ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
