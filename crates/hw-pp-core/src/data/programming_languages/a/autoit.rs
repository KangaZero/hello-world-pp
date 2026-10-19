//! Hello-world snippet for `ProgrammingLanguageName::Autoit`.
//!
//! Imported from leachim6/hello-world (`a/AutoIt.au3` at `a152253`),
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
    language: Autoit => "AutoIt",
    versions: [Autoit => "AutoIt"],
    extensions: [Au3 => "au3"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Autoit,
        extensions: &[FileExtension::Au3],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Autoit,
            syntax: concat!(
                r#"MsgBox(0, "Message Box", ""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"MsgBox(0, "Message Box", ""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
