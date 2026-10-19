//! Hello-world snippet for `ProgrammingLanguageName::Carbon`.
//!
//! Imported from leachim6/hello-world (`c/Carbon.carbon` at `a152253`),
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
    language: Carbon => "Carbon",
    versions: [Carbon => "Carbon"],
    extensions: [Carbon => "carbon"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Carbon,
        extensions: &[FileExtension::Carbon],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Carbon,
            syntax: concat!(
                r#"package HelloWorld api;

fn Main() -> i32 {
  Print(""#,
                output_for_hello_world!(),
                r#"");
  return 0;
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"package HelloWorld api;

fn Main() -> i32 {
  Print(""#,
                    output_for_hello_world!(),
                    r#"");
  return 0;
}"#
                ),
            )],
        }],
    };
