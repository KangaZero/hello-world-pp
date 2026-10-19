//! Hello-world snippet for `ProgrammingLanguageName::Dockerfile`.
//!
//! Imported from leachim6/hello-world (PR #1681 `d/dockerfile`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dockerfile => "Dockerfile",
    versions: [Dockerfile => "Dockerfile"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dockerfile,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dockerfile,
            syntax: concat!(
                r#"FROM alpine:latest
CMD echo ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"FROM alpine:latest
CMD echo ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
