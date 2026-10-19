//! Hello-world snippet for `ProgrammingLanguageName::Abc`.
//!
//! Imported from leachim6/hello-world (`a/ABC.abc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ABC.abc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Educational, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Abc => "ABC",
    versions: [Abc => "ABC"],
    extensions: [Abc => "abc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Abc,
        extensions: &[FileExtension::Abc],
        categories: &[Historical, Educational],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Abc,
            syntax: concat!(r#"WRITE ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"WRITE ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
