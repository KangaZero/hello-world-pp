//! Hello-world snippet for `ProgrammingLanguageName::False`.
//!
//! Imported from leachim6/hello-world (`f/False.f` at `a152253`),
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
    language: False => "False",
    versions: [False => "False"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::False,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::False,
            syntax: concat!(
                r#"""#,
                output_for_hello_world!(),
                r#"
""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"""#,
                    output_for_hello_world!(),
                    r#"
""#
                ),
            )],
        }],
    };
