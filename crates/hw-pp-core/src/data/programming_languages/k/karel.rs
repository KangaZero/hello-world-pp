//! Hello-world snippet for `ProgrammingLanguageName::Karel`.
//!
//! Imported from leachim6/hello-world (`k/Karel.kl` at `a152253`),
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
    language: Karel => "Karel",
    versions: [Karel => "Karel"],
    extensions: [Kl => "kl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Karel,
        extensions: &[FileExtension::Kl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Karel,
            syntax: concat!(
                r#"PROGRAM hello_world
BEGIN
  WRITE(""#,
                output_for_hello_world!(),
                r#"", CR)
END hello_world"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"PROGRAM hello_world
BEGIN
  WRITE(""#,
                    output_for_hello_world!(),
                    r#"", CR)
END hello_world"#
                ),
            )],
        }],
    };
