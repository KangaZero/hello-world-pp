//! Hello-world snippet for `ProgrammingLanguageName::Ksh`.
//!
//! Imported from leachim6/hello-world (`k/KSH.ksh` at `a152253`),
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
    language: Ksh => "KSH",
    versions: [Ksh => "KSH"],
    extensions: [Ksh => "ksh"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ksh,
        extensions: &[FileExtension::Ksh],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ksh,
            syntax: concat!(
                r#"#!/bin/ksh
echo ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#!/bin/ksh
echo ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
