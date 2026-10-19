//! Hello-world snippet for `ProgrammingLanguageName::Graphql`.
//!
//! Imported from leachim6/hello-world (`g/GraphQL.graphql` at `a152253`),
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
    language: Graphql => "GraphQL",
    versions: [Graphql => "GraphQL"],
    extensions: [Graphql => "graphql"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Graphql,
        extensions: &[FileExtension::Graphql],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Graphql,
            syntax: concat!(
                r#"{
    ""#,
                output_for_hello_world!(),
                r#""
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"{
    ""#,
                    output_for_hello_world!(),
                    r#""
}"#
                ),
            )],
        }],
    };
