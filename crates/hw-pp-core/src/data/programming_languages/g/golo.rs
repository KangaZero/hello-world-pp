//! Hello-world snippet for `ProgrammingLanguageName::Golo`.
//!
//! Imported from leachim6/hello-world (`g/Golo.golo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Golo => "Golo",
    versions: [Golo => "Golo"],
    extensions: [Golo => "golo"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Golo,
        extensions: &[FileExtension::Golo],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Golo,
            syntax: concat!(
                r#"module hello.world

function main = |args| {
  println(""#,
                output_for_hello_world!(),
                r#"")
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"module hello.world

function main = |args| {
  println(""#,
                    output_for_hello_world!(),
                    r#"")
}"#
                ),
            )],
        }],
    };
