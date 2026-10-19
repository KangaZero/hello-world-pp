//! Hello-world snippet for `ProgrammingLanguageName::Pharo`.
//!
//! Imported from leachim6/hello-world (`p/Pharo.st` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pharo.st>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pharo,
            syntax: concat!(r"'", output_for_hello_world!(), r"' crLog"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"'", output_for_hello_world!(), r"' crLog"),
            )],
        }],
    };
