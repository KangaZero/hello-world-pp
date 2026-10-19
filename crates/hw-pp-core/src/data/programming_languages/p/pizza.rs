//! Hello-world snippet for `ProgrammingLanguageName::Pizza`.
//!
//! Imported from leachim6/hello-world (`p/Pizza.pizza` at `a152253`),
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
    language: Pizza => "Pizza",
    versions: [Pizza => "Pizza"],
    extensions: [Pizza => "pizza"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pizza,
        extensions: &[FileExtension::Pizza],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pizza,
            syntax: concat!(
                r#"class HelloWorld {
  public static void main(String[] args) {
    System.out.println(""#,
                output_for_hello_world!(),
                r#"");
  }
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"class HelloWorld {
  public static void main(String[] args) {
    System.out.println(""#,
                    output_for_hello_world!(),
                    r#"");
  }
}"#
                ),
            )],
        }],
    };
