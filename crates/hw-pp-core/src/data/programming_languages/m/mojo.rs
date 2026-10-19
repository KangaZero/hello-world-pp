//! Hello-world snippet for `ProgrammingLanguageName::Mojo`.
//!
//! Imported from leachim6/hello-world (`m/Mojo.🔥` at `a152253`),
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
    language: Mojo => "Mojo",
    versions: [Mojo => "Mojo"],
    extensions: [U1f525 => "🔥"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mojo,
        extensions: &[FileExtension::U1f525],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mojo,
            syntax: concat!(
                r#"fn main():
  print(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"fn main():
  print(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
