//! Hello-world snippet for `ProgrammingLanguageName::Koka`.
//!
//! Imported from leachim6/hello-world (`k/Koka.kk` at `a152253`),
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
    language: Koka => "Koka",
    versions: [Koka => "Koka"],
    extensions: [Kk => "kk"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Koka,
        extensions: &[FileExtension::Kk],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Koka,
            syntax: concat!(
                r#"fun main() {
  println(""#,
                output_for_hello_world!(),
                r#"")
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"fun main() {
  println(""#,
                    output_for_hello_world!(),
                    r#"")
}"#
                ),
            )],
        }],
    };
