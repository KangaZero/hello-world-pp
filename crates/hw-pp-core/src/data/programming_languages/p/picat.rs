//! Hello-world snippet for `ProgrammingLanguageName::Picat`.
//!
//! Imported from leachim6/hello-world (`p/PiCat.pi` at `a152253`),
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
    language: Picat => "PiCat",
    versions: [Picat => "PiCat"],
    extensions: [Pi => "pi"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Picat,
        extensions: &[FileExtension::Pi],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Picat,
            syntax: concat!(
                r#"main =>
  println(""#,
                output_for_hello_world!(),
                r#"")."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"main =>
  println(""#,
                    output_for_hello_world!(),
                    r#"")."#
                ),
            )],
        }],
    };
