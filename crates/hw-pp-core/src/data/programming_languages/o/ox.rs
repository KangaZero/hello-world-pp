//! Hello-world snippet for `ProgrammingLanguageName::Ox`.
//!
//! Imported from leachim6/hello-world (`o/OX.oz` at `a152253`),
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
    language: Ox => "OX",
    versions: [Ox => "OX"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ox,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ox,
            syntax: concat!(r"{Show '", output_for_hello_world!(), r"'}"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"{Show '", output_for_hello_world!(), r"'}"),
            )],
        }],
    };
