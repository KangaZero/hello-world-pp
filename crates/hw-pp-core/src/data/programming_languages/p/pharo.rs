//! Hello-world snippet for `ProgrammingLanguageName::Pharo`.
//!
//! Imported from leachim6/hello-world (`p/Pharo.st` at `a152253`),
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
    language: Pharo => "Pharo",
    versions: [Pharo => "Pharo"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pharo,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pharo,
            syntax: concat!(r"'", output_for_hello_world!(), r"' crLog"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"'", output_for_hello_world!(), r"' crLog"),
            )],
        }],
    };
