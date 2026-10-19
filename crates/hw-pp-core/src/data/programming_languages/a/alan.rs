//! Hello-world snippet for `ProgrammingLanguageName::Alan`.
//!
//! Imported from leachim6/hello-world (`a/Alan.ln` at `a152253`),
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
    language: Alan => "Alan",
    versions: [Alan => "Alan"],
    extensions: [Ln => "ln"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Alan,
        extensions: &[FileExtension::Ln],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Alan,
            syntax: concat!(
                r#"import @std/app

on app.start {
  app.print(""#,
                output_for_hello_world!(),
                r#"");
  emit app.exit 0;
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"import @std/app

on app.start {
  app.print(""#,
                    output_for_hello_world!(),
                    r#"");
  emit app.exit 0;
}"#
                ),
            )],
        }],
    };
