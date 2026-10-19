//! Hello-world snippet for `ProgrammingLanguageName::Elvish`.
//!
//! Imported from leachim6/hello-world (`e/Elvish.elv` at `a152253`),
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
    language: Elvish => "Elvish",
    versions: [Elvish => "Elvish"],
    extensions: [Elv => "elv"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elvish,
        extensions: &[FileExtension::Elv],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Elvish,
            syntax: concat!(
                r#"#!/usr/bin/env elvish
echo ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#!/usr/bin/env elvish
echo ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
