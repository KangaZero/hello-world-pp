//! Hello-world snippet for `ProgrammingLanguageName::Skip`.
//!
//! Imported from leachim6/hello-world (`s/Skip.sk` at `a152253`),
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
    language: Skip => "Skip",
    versions: [Skip => "Skip"],
    extensions: [Sk => "sk"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Skip,
        extensions: &[FileExtension::Sk],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Skip,
            syntax: concat!(
                r#"fun main(): void {
  print_string(""#,
                output_for_hello_world!(),
                r#"")
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"fun main(): void {
  print_string(""#,
                    output_for_hello_world!(),
                    r#"")
}"#
                ),
            )],
        }],
    };
