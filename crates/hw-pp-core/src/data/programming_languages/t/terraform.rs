//! Hello-world snippet for `ProgrammingLanguageName::Terraform`.
//!
//! Imported from leachim6/hello-world (`t/Terraform.tf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Terraform.tf>
//! - <https://en.wikipedia.org/wiki/Terraform_(software)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Declarative;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Configuration,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Terraform => "Terraform",
    versions: [Terraform => "Terraform"],
    extensions: [Tf => "tf"],
}

const TERRAFORM: &str = concat!(
    r#"output "msg" {
  value = ""#,
    output_for_hello_world!(),
    r#""
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Terraform,
        extensions: &[FileExtension::Tf],
        categories: &[Configuration],
        families: &[],
        paradigms: &[Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Terraform,
            syntax: TERRAFORM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TERRAFORM,
                    [
                        (TokenKind::Plain, r"output "),
                        (TokenKind::String, r#""msg""#),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  value = ",
                        ),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
