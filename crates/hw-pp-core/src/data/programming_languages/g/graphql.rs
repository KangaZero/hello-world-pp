//! Hello-world snippet for `ProgrammingLanguageName::Graphql`.
//!
//! Imported from leachim6/hello-world (`g/GraphQL.graphql` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/GraphQL.graphql>
//! - <https://en.wikipedia.org/wiki/GraphQL> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Declarative;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Query,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Graphql => "GraphQL",
    versions: [Graphql => "GraphQL"],
    extensions: [Graphql => "graphql"],
}

const GRAPHQL: &str = concat!(
    r#"{
    ""#,
    output_for_hello_world!(),
    r#""
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Graphql,
        extensions: &[FileExtension::Graphql],
        categories: &[Query],
        families: &[],
        paradigms: &[Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Graphql,
            syntax: GRAPHQL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GRAPHQL,
                    [
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
