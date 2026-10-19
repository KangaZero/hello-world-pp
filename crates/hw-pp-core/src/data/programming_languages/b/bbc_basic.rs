//! Hello-world snippet for `ProgrammingLanguageName::BbcBasic`.
//!
//! Imported from leachim6/hello-world (`b/BBC BASIC.bbc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BBC%20BASIC.bbc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Historical,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: BbcBasic => "BBC BASIC",
    versions: [BbcBasic => "BBC BASIC"],
    extensions: [Bbc => "bbc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BbcBasic,
        extensions: &[FileExtension::Bbc],
        categories: &[Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BbcBasic,
            syntax: concat!(r#"PRINT ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"PRINT ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
