//! Hello-world snippet for `ProgrammingLanguageName::Llrl`.
//!
//! Imported from leachim6/hello-world (`l/llrl.llrl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/llrl.llrl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Llrl => "llrl",
    versions: [Llrl => "llrl"],
    extensions: [Llrl => "llrl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Llrl,
        extensions: &[FileExtension::Llrl],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Llrl,
            syntax: concat!(r#"(println! ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"(println! ""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
