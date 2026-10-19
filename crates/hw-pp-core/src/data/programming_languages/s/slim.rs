//! Hello-world snippet for `ProgrammingLanguageName::Slim`.
//!
//! Imported from leachim6/hello-world (`s/Slim.slim` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Slim.slim>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Slim => "Slim",
    versions: [Slim => "Slim"],
    extensions: [Slim => "slim"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Slim,
        extensions: &[FileExtension::Slim],
        categories: &[Markup],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Slim,
            syntax: concat!(
                r"doctype html
head
  title ",
                output_for_hello_world!(),
                r"
body
  h1 ",
                output_for_hello_world!()
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"doctype html
head
  title ",
                    output_for_hello_world!(),
                    r"
body
  h1 ",
                    output_for_hello_world!()
                ),
            )],
        }],
    };
