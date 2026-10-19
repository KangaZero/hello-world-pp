//! Hello-world snippet for `ProgrammingLanguageName::Dog`.
//!
//! Imported from leachim6/hello-world (`d/DOG` at `a152253`),
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
    language: Dog => "DOG",
    versions: [Dog => "DOG"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dog,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dog,
            syntax: concat!(r#"bark ""#, output_for_hello_world!(), r#"\n""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"bark ""#, output_for_hello_world!(), r#"\n""#),
            )],
        }],
    };
