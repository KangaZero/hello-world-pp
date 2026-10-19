//! Hello-world snippet for `ProgrammingLanguageName::Zhpy`.
//!
//! Imported from leachim6/hello-world (`z/Zhpy.py` at `a152253`),
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
    language: Zhpy => "Zhpy",
    versions: [Zhpy => "Zhpy"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zhpy,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zhpy,
            syntax: concat!(r"印出 '", output_for_hello_world!(), r"'"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"印出 '", output_for_hello_world!(), r"'"),
            )],
        }],
    };
