//! Hello-world snippet for `ProgrammingLanguageName::N8th`.
//!
//! Imported from leachim6/hello-world (`#/8th` at `a152253`),
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
    language: N8th => "8th",
    versions: [N8th => "8th"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N8th,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N8th,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#"\n" ."#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#"\n" ."#),
            )],
        }],
    };
