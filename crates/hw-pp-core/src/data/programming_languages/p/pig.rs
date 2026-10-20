//! Hello-world snippet for `ProgrammingLanguageName::Pig`.
//!
//! Imported from leachim6/hello-world (`p/Pig.pig` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pig.pig>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Query,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Pig => "Pig",
    versions: [Pig => "Pig"],
    extensions: [Pig => "pig"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pig,
        extensions: &[FileExtension::Pig],
        categories: &[Query],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pig,
            syntax: concat!(output_for_hello_world!(), r"PIG", output_for_hello_world!()),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(output_for_hello_world!(), r"PIG", output_for_hello_world!()),
            )],
        }],
    };
