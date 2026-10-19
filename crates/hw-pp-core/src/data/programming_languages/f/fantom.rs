//! Hello-world snippet for `ProgrammingLanguageName::Fantom`.
//!
//! Imported from leachim6/hello-world (`f/Fantom.fan` at `a152253`),
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
    language: Fantom => "Fantom",
    versions: [Fantom => "Fantom"],
    extensions: [Fan => "fan"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fantom,
        extensions: &[FileExtension::Fan],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fantom,
            syntax: concat!(
                r#"// Hello from Fantom!
class HelloWorld {
  static Void main() {
    echo(""#,
                output_for_hello_world!(),
                r#"")
  }
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"// Hello from Fantom!
class HelloWorld {
  static Void main() {
    echo(""#,
                    output_for_hello_world!(),
                    r#"")
  }
}"#
                ),
            )],
        }],
    };
