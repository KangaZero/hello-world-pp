//! Hello-world snippet for `ProgrammingLanguageName::Snowman`.
//!
//! Imported from leachim6/hello-world (`s/Snowman.sm` at `a152253`),
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
    language: Snowman => "Snowman",
    versions: [Snowman => "Snowman"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Snowman,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Snowman,
            syntax: concat!(r#"~""#, output_for_hello_world!(), r#""sPvG"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"~""#, output_for_hello_world!(), r#""sPvG"#),
            )],
        }],
    };
