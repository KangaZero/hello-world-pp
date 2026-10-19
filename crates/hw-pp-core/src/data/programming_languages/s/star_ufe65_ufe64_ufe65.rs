//! Hello-world snippet for `ProgrammingLanguageName::StarUfe65Ufe64Ufe65`.
//!
//! Imported from leachim6/hello-world (`#/∗﹥﹤﹥` at `a152253`),
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
    language: StarUfe65Ufe64Ufe65 => "∗﹥﹤﹥",
    versions: [StarUfe65Ufe64Ufe65 => "∗﹥﹤﹥"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::StarUfe65Ufe64Ufe65,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::StarUfe65Ufe64Ufe65,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#""r>Ool?u!|;"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#""r>Ool?u!|;"#),
            )],
        }],
    };
