//! Hello-world snippet for `ProgrammingLanguageName::StandardMl`.
//!
//! Imported from leachim6/hello-world (`s/Standard ML.sml` at `a152253`),
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
    language: StandardMl => "Standard ML",
    versions: [StandardMl => "Standard ML"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::StandardMl,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::StandardMl,
            syntax: concat!(
                r#"fun hello() = print(""#,
                output_for_hello_world!(),
                r#"\n");

hello()"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"fun hello() = print(""#,
                    output_for_hello_world!(),
                    r#"\n");

hello()"#
                ),
            )],
        }],
    };
