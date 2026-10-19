//! Hello-world snippet for `ProgrammingLanguageName::Comefrom0x10`.
//!
//! Imported from leachim6/hello-world (`c/Comefrom0x10` at `a152253`),
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
    language: Comefrom0x10 => "Comefrom0x10",
    versions: [Comefrom0x10 => "Comefrom0x10"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Comefrom0x10,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Comefrom0x10,
            syntax: concat!(r"'", output_for_hello_world!(), r"'"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"'", output_for_hello_world!(), r"'"),
            )],
        }],
    };
