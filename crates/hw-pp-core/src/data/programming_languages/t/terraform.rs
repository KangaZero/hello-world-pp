//! Hello-world snippet for `ProgrammingLanguageName::Terraform`.
//!
//! Imported from leachim6/hello-world (`t/Terraform.tf` at `a152253`),
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
    language: Terraform => "Terraform",
    versions: [Terraform => "Terraform"],
    extensions: [Tf => "tf"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Terraform,
        extensions: &[FileExtension::Tf],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Terraform,
            syntax: concat!(
                r#"output "msg" {
  value = ""#,
                output_for_hello_world!(),
                r#""
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"output "msg" {
  value = ""#,
                    output_for_hello_world!(),
                    r#""
}"#
                ),
            )],
        }],
    };
