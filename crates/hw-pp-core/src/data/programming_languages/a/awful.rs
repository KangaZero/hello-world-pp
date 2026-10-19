//! Hello-world snippet for `ProgrammingLanguageName::Awful`.
//!
//! Imported from leachim6/hello-world (`a/Awful.yuk` at `a152253`),
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
    language: Awful => "Awful",
    versions: [Awful => "Awful"],
    extensions: [Yuk => "yuk"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Awful,
        extensions: &[FileExtension::Yuk],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Awful,
            syntax: concat!(
                r#"#!/usr/bin/awful

:writeln s""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#!/usr/bin/awful

:writeln s""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
