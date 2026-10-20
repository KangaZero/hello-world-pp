//! Hello-world snippet for `ProgrammingLanguageName::Pizza`.
//!
//! Imported from leachim6/hello-world (`p/Pizza.pizza` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pizza.pizza>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Java,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Compiled, Historical],
        families: &[Java],
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
