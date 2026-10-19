//! Hello-world snippet for `ProgrammingLanguageName::Odin`.
//!
//! Imported from leachim6/hello-world (`o/Odin.odin` at `a152253`),
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
    language: Odin => "Odin",
    versions: [Odin => "Odin"],
    extensions: [Odin => "odin"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Odin,
        extensions: &[FileExtension::Odin],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Odin,
            syntax: concat!(
                r#"package main

import "core:fmt"

main :: proc() {
  fmt.println(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"package main

import "core:fmt"

main :: proc() {
  fmt.println(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };
