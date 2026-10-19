//! Hello-world snippet for `ProgrammingLanguageName::Gravity`.
//!
//! Imported from leachim6/hello-world (`g/Gravity.gravity` at `a152253`),
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
    language: Gravity => "Gravity",
    versions: [Gravity => "Gravity"],
    extensions: [Gravity => "gravity"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gravity,
        extensions: &[FileExtension::Gravity],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gravity,
            syntax: concat!(
                r#"func main() {
  System.print(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"func main() {
  System.print(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };
