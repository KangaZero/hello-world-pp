//! Hello-world snippet for `ProgrammingLanguageName::N0587`.
//!
//! Imported from leachim6/hello-world (`#/0587` at `a152253`),
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
    language: N0587 => "0587",
    versions: [N0587 => "0587"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N0587,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N0587,
            syntax: concat!(r"04(", output_for_hello_world!(), r")"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"04(", output_for_hello_world!(), r")"),
            )],
        }],
    };
