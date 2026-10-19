//! Hello-world snippet for `ProgrammingLanguageName::Clio`.
//!
//! Imported from leachim6/hello-world (`c/Clio.clio` at `a152253`),
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
    language: Clio => "Clio",
    versions: [Clio => "Clio"],
    extensions: [Clio => "clio"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clio,
        extensions: &[FileExtension::Clio],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clio,
            syntax: concat!(
                r#"export fn main argv:
  ""#,
                output_for_hello_world!(),
                r#"" -> console.log"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"export fn main argv:
  ""#,
                    output_for_hello_world!(),
                    r#"" -> console.log"#
                ),
            )],
        }],
    };
