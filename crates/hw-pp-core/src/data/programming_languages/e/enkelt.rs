//! Hello-world snippet for `ProgrammingLanguageName::Enkelt`.
//!
//! Imported from leachim6/hello-world (`e/Enkelt.e` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Enkelt.e>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Enkelt => "Enkelt",
    versions: [Enkelt => "Enkelt"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Enkelt,
        extensions: &[],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Enkelt,
            syntax: concat!(r#"skriv(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"skriv(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
