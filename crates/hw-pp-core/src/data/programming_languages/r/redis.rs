//! Hello-world snippet for `ProgrammingLanguageName::Redis`.
//!
//! Imported from leachim6/hello-world (`r/Redis` at `a152253`),
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
    language: Redis => "Redis",
    versions: [Redis => "Redis"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Redis,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Redis,
            syntax: concat!(r"set ", output_for_hello_world!()),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"set ", output_for_hello_world!()),
            )],
        }],
    };
