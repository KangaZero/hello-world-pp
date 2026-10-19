//! Hello-world snippet for `ProgrammingLanguageName::Gleam`.
//!
//! Imported from leachim6/hello-world (`g/Gleam.gleam` at `a152253`),
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
    language: Gleam => "Gleam",
    versions: [Gleam => "Gleam"],
    extensions: [Gleam => "gleam"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gleam,
        extensions: &[FileExtension::Gleam],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gleam,
            syntax: concat!(
                r#"import gleam/io

pub fn main() {
  io.println(""#,
                output_for_hello_world!(),
                r#"")
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"import gleam/io

pub fn main() {
  io.println(""#,
                    output_for_hello_world!(),
                    r#"")
}"#
                ),
            )],
        }],
    };
